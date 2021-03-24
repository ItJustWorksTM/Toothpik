/*
 * Copyright 2020 ItJustWorksTM
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use uuid::Uuid;

use crate::structs::user::{Error, User, UserId, UserName};

use super::*;
use sqlx::pool::PoolConnection;
use tokio::sync::Mutex;

#[cfg(any(feature = "mail_check", feature = "reg_captcha"))]
use crate::definitions::*;
#[cfg(feature = "reg_captcha")]
use crate::structs::captcha::HCaptchaResponse;
#[cfg(feature = "mail_check")]
use crate::structs::user::UserValidation;
#[cfg(feature = "mail_check")]
use lettre::message::Mailbox;
#[cfg(feature = "mail_check")]
use lettre::{Message, SmtpTransport, Transport};
#[cfg(feature = "mail_check")]
use rand::random;
#[cfg(feature = "reg_captcha")]
use std::env;

pub async fn features(
    req: Request<()>,
    client: Arc<AsyncClient>,
    _: Arc<Mutex<PoolConnection<Sqlite>>>,
) -> Option<()> {
    const COUNT_FEATURES: usize =
        cfg!(feature = "reg_captcha") as usize + cfg!(feature = "mail_check") as usize;
    client
        .publish(
            req.reply_topic,
            QoS::AtLeastOnce,
            false,
            serde_cbor::to_vec::<[&str; COUNT_FEATURES]>(&[
                #[cfg(feature = "reg_captcha")]
                "reg_captcha",
                #[cfg(feature = "mail_check")]
                "mail_check",
            ])
            .ok()?,
        )
        .await
        .ok()?;
    Some(())
}

pub async fn register(
    req: Request<User>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
    #[cfg(feature = "mail_check")] mailer: Arc<Mutex<SmtpTransport>>,
) -> Option<()> {
    match req.obj {
        Some(obj) => {
            #[cfg(feature = "reg_captcha")]
            {
                let http_client = reqwest::Client::new();
                let captcha_req = http_client
                    .post(CAPTCHA_VERIFY_URL)
                    .form(&[
                        (
                            "secret",
                            env::var("CAPTCHA_DEFAULT_SECRET")
                                .unwrap_or(CAPTCHA_DEFAULT_SECRET.into()),
                        ),
                        ("response", obj.captcha_token),
                    ])
                    .send()
                    .await;
                if let Some(err) = match captcha_req {
                    Ok(res) => match res.json::<HCaptchaResponse>().await {
                        Ok(res) if res.success => None,
                        Ok(res)
                            if res
                                .error_codes
                                .as_ref()?
                                .contains(&String::from("invalid-or-already-seen-response")) =>
                        {
                            Some("Captcha token already used")
                        }
                        Ok(res)
                            if res
                                .error_codes
                                .as_ref()?
                                .contains(&String::from("invalid-input-response")) =>
                        {
                            Some("Captcha token is invalid")
                        }
                        Ok(res) => {
                            println!("Captcha request has issues:\n\t{:?}", res);
                            Some("Captcha token verification unsuccessful")
                        }
                        Err(e) => {
                            println!("Failed to deserialise /siteverify response:\n\t{}", e);
                            Some("Captcha token verification failed")
                        }
                    },
                    Err(e) => {
                        println!("Captcha verification seems down:\n\t{}", e);
                        Some("Captcha token verification unreachable")
                    }
                } {
                    return send_err(&req.reply_topic, err, client).await;
                }
            }

            let uuid = Uuid::new_v4().to_string();
            #[cfg(feature = "mail_check")]
            let random_code = random::<i32>().abs();
            #[cfg(feature = "mail_check")]
            let email_code = if random_code != 0 {
                random_code
            } else {
                0xBADBEE
            };
            #[cfg(not(feature = "mail_check"))]
            let email_code = 0;

            let result = {
                sqlx::query(
                    r#"
                    INSERT INTO users (id, name, username, email, secret, verification_code)
                    VALUES(?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(&uuid)
                .bind(&obj.name)
                .bind(obj.username)
                .bind(&obj.email)
                .bind(obj.secret)
                .bind(&email_code)
                .execute(&mut *data.lock().await)
                .await
            };

            client
                .publish(
                    &req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    match result {
                        Ok(_) => {
                            #[cfg(feature = "mail_check")]
                            {
                                let destination = match obj.email.parse() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        return send_err(
                                            &req.reply_topic,
                                            "Invalid Email Address",
                                            client,
                                        )
                                        .await
                                    }
                                };

                                let email = match Message::builder()
                                    .from(SMTP_SENDER.clone())
                                    .to(Mailbox::new(Some(obj.name), destination))
                                    .subject("Email validation code")
                                    .body(format!("{:X}", email_code))
                                {
                                    Ok(v) => v,
                                    Err(_) => {
                                        return send_err(
                                            &req.reply_topic,
                                            "Invalid Email Address",
                                            client,
                                        )
                                        .await
                                    }
                                };

                                match (*mailer.lock().await).send(&email) {
                                    Ok(_) => serde_cbor::to_vec(&UserId { id: uuid }),
                                    Err(e) => {
                                        println!("Could not send email: {:?}", e);
                                        serde_cbor::to_vec(&Error {
                                            error: "Unable to verify".into(),
                                        })
                                    }
                                }
                            }
                            #[cfg(not(feature = "mail_check"))]
                            serde_cbor::to_vec(&UserId { id: uuid })
                        }
                        Err(e) => serde_cbor::to_vec(&Error {
                            error: match e {
                                sqlx::Error::Database(e) => e.message().into(),
                                _ => "Registration failed".into(),
                            },
                        }),
                    }
                    .ok()?,
                )
                .await
                .ok()?;
        }
        None => send_err(&req.reply_topic, "Invalid User Format", client).await?,
    }
    Some(())
}

#[cfg(feature = "mail_check")]
pub async fn validate(
    req: Request<UserValidation>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
) -> Option<()> {
    match req.obj {
        Some(obj) => {
            let mut data_lk = data.lock().await;
            let id_code = {
                sqlx::query_as::<_, (String, i32)>(
                    "SELECT id, verification_code FROM users WHERE username = ?",
                )
                .bind(obj.username.clone())
                .fetch_one(&mut *data_lk)
                .await
            };
            client
                .publish(
                    req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    match id_code {
                        Ok((user_id, verification_code)) => {
                            if obj.verification_code == verification_code {
                                sqlx::query("UPDATE users SET verification_code = 0 WHERE id = ?")
                                    .bind(&user_id)
                                    .execute(&mut *data_lk)
                                    .await
                                    .ok()?;
                                println!("Validated {}", obj.username);
                                serde_cbor::to_vec(&UserId { id: user_id })
                            } else {
                                serde_cbor::to_vec(&Error {
                                    error: "Code is incorrect".into(),
                                })
                            }
                        }
                        Err(_) => serde_cbor::to_vec(&Error {
                            error: "Could not retrieve user".into(),
                        }),
                    }
                    .ok()?,
                )
                .await
                .ok()?;
        }
        None => send_err(&req.reply_topic, "Invalid Format", client).await?,
    };
    Some(())
}

pub async fn get_self(
    req: Request<()>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
) -> Option<()> {
    let user = {
        sqlx::query_as::<_, User>(
            "SELECT id, name, username, email, secret FROM users WHERE id = ?",
        )
        .bind(&req.id)
        .fetch_one(&mut *data.lock().await)
        .await
    };
    client
        .publish(
            req.reply_topic,
            QoS::AtLeastOnce,
            false,
            match user {
                Ok(user) => serde_cbor::to_vec(&user),
                Err(_) => serde_cbor::to_vec(&Error {
                    error: "Could not retrieve user".into(),
                }),
            }
            .ok()?,
        )
        .await
        .ok()?;
    Some(())
}

pub async fn get_id(
    req: Request<UserName>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
) -> Option<()> {
    match req.obj {
        Some(obj) => {
            let user = {
                sqlx::query_as::<_, UserId>("SELECT id FROM users WHERE username = ?")
                    .bind(obj.username)
                    .fetch_one(&mut *data.lock().await)
                    .await
            };
            client
                .publish(
                    req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    match user {
                        Ok(user) => serde_cbor::to_vec(&user),
                        Err(_) => serde_cbor::to_vec(&Error {
                            error: "Could not retrieve user".into(),
                        }),
                    }
                    .ok()?,
                )
                .await
                .ok()?;
        }
        None => send_err(&req.reply_topic, "Invalid Format", client).await?,
    };
    Some(())
}
