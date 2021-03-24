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

use sqlx::sqlite::SqliteDone;
use store_utils::structs::AvailOpeningHours;
use tokio::sync::RwLock;
use uuid::Uuid;

use availability::is_available;

use crate::structs::appointment::*;

use super::*;

async fn create_appointment(
    obj: &SQLAppointment,
    data: &mut PoolConnection<Sqlite>,
) -> Result<SqliteDone, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO appointments (id, userid, dentistid, date, reason, mobile, email, name)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&Uuid::new_v4().to_string())
    .bind(&obj.userid)
    .bind(&obj.dentistid)
    .bind(&obj.date)
    .bind(&obj.reason)
    .bind(&obj.mobile)
    .bind(&obj.email)
    .bind(&obj.name)
    .execute(data)
    .await
}

// Generic booking
pub async fn book<T: serde::de::DeserializeOwned>(
    sqlobj: Option<SQLAppointment>,
    requestid: String,
    req: Request<T>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
    registry: Arc<RwLock<Vec<AvailOpeningHours>>>,
) -> Option<()> {
    match sqlobj {
        Some(obj) => {
            let mut data_lk = data.lock().await;
            client
                .publish(
                    req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    &*serialize(
                        req.serial_type,
                        &AppointmentResponse {
                            userid: obj.userid.clone(),
                            requestid: requestid.clone(),
                            time: match is_available(
                                obj.dentistid as i64,
                                obj.date,
                                &*registry.read().await,
                                &mut *data_lk,
                            )
                            .await
                            {
                                Some(update) => {
                                    match create_appointment(&obj, &mut data_lk).await {
                                        Ok(_) => {
                                            // Only send out if it becomes unavailable
                                            if true {
                                                client
                                            .publish(
                                                "store/appointment/public/realtime/availability",
                                                QoS::AtLeastOnce,
                                                false,
                                                &*serialize(req.serial_type, &update)?,
                                            )
                                            .await
                                            .ok()?;
                                            }
                                            // Take time aligned to slots
                                            format!("{} {}", update.date, update.time)
                                        }
                                        Err(_) => "none".into(),
                                    }
                                }
                                None => "none".into(),
                            },
                        },
                    )?,
                )
                .await
                .ok()?;
        }
        None => return send_err(&req.reply_topic, "Invalid format", req.serial_type, client).await,
    }
    Some(())
}

pub async fn qbook(
    req: Request<QAppointment>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
    registry: Arc<RwLock<Vec<AvailOpeningHours>>>,
) -> Option<()> {
    match req.obj.clone() {
        Some(obj) => {
            let requestid = obj.requestid.clone();
            book(
                QAppointment::sql(obj),
                requestid,
                req,
                client,
                data,
                registry,
            )
            .await?;
        }
        None => send_err(&req.reply_topic, "Invalid Format", req.serial_type, client).await?,
    };
    Some(())
}

pub async fn nbook(
    req: Request<NAppointment>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
    registry: Arc<RwLock<Vec<AvailOpeningHours>>>,
) -> Option<()> {
    match req.obj.clone() {
        Some(obj) => {
            book(
                NAppointment::sql(obj),
                "null".into(),
                req,
                client,
                data,
                registry,
            )
            .await?;
        }
        None => send_err(&req.reply_topic, "Invalid Format", req.serial_type, client).await?,
    };
    Some(())
}
