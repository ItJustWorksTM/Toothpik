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

use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::atomic::{AtomicI64, Ordering};
use store_utils::has_table;
use uuid::Uuid;

use definitions::*;
use tokio::sync::Mutex;

mod definitions;
mod handling;
mod structs;

#[cfg(feature = "mail_check")]
use lettre::transport::smtp::authentication::Credentials;
#[cfg(feature = "mail_check")]
use lettre::SmtpTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let debug = env::var("DEBUG").is_ok();
    println!("Debug: {}", debug);

    let pool = SqlitePoolOptions::new()
        .max_connections(1) // Set to 1 to avoid table lock, unsure how to do concurrent SQL
        .connect_timeout(Duration::from_secs(10))
        .connect(&env::var("DATABASE_URL").unwrap_or("sqlite://:memory:".into()))
        .await?;

    let mut conn = pool.acquire().await?;
    if !has_table(&mut conn, "users").await {
        println!("Creating user table");
        sqlx::query(
            r#"
            CREATE TABLE users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            secret TEXT NOT NULL,
            verification_code INTEGER NOT NULL);
            "#,
        )
        .execute(&mut conn)
        .await?;
    }

    let mut mqtt_options = MqttOptions::new(
        format!("{}", Uuid::new_v4()),
        &env::var("BROKER_URL").unwrap_or(MQTT_DEFAULT.into()),
        1883,
    );
    mqtt_options.set_credentials(MQTT_USER, MQTT_PW);
    mqtt_options.set_keep_alive(400);

    println!("Connecting on: {}", mqtt_options.broker_address().0);

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe(MQTT_SUB_TOPIC, QoS::AtMostOnce).await?;

    #[cfg(feature = "mail_check")]
    let mailer = SmtpTransport::relay(SMTP_RELAY_DOMAIN)?
        .credentials(Credentials::new(SMTP_USER.to_string(), SMTP_PW.to_string()))
        .build();

    let cl = Arc::new(client);
    let db = Arc::new(Mutex::new(conn));
    #[cfg(feature = "mail_check")]
    let ml = Arc::new(Mutex::new(mailer));

    let inflight = Arc::new(AtomicI64::new(0));
    let inflight_limit: i64 = match env::var("INFLIGHT_LIMIT") {
        Ok(var) => var.parse().unwrap_or(INFLIGHT_LIMIT_DEFAULT),
        Err(_) => INFLIGHT_LIMIT_DEFAULT,
    };

    loop {
        match eventloop.poll().await {
            Ok(event) => match event {
                Event::Incoming(Incoming::Publish(p)) => {
                    if inflight.load(Ordering::SeqCst) < inflight_limit {
                        let (inflight, cl, db) = (inflight.clone(), cl.clone(), db.clone());
                        #[cfg(feature = "mail_check")]
                        let ml = ml.clone();
                        tokio::spawn(async move {
                            inflight.fetch_add(1, Ordering::SeqCst);
                            handling::publish::handle_pub(
                                p,
                                cl,
                                db,
                                #[cfg(feature = "mail_check")]
                                ml,
                            )
                            .await;
                            inflight.fetch_sub(1, Ordering::SeqCst);
                        });
                    } else {
                        println!("Inflight limit reached!");
                    }
                }
                _ => (),
            },
            Err(e) => {
                println!("Error: {:?}", e);
                tokio::time::delay_for(Duration::from_secs(5)).await;
            }
        }
    }
}
