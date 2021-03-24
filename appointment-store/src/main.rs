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

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use sqlx::sqlite::SqlitePool;
use store_utils::has_table;
use store_utils::request::{deserialize, mqtt_request, SerialType};
use store_utils::structs::{AvailOpeningHours, Registry};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use definitions::*;
use std::sync::atomic::{AtomicI64, Ordering};

mod definitions;
mod handling;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap_or("sqlite://:memory:".into()))
        .await?;
    let mut conn = pool.acquire().await?;

    if !has_table(&mut conn, "appointments").await {
        sqlx::query(
            r#"
            CREATE TABLE appointments (
            id TEXT PRIMARY KEY UNIQUE,
            userid TEXT NOT NULL,
            dentistid INTEGER NOT NULL,
            date DATETIME NOT NULL,
            reason TEXT,
            mobile TEXT,
            email TEXT,
            name TEXT);
            "#,
        )
        .execute(&mut conn)
        .await?;
    }
    let uuid = Uuid::new_v4().to_string();
    let mut mqtt_options = MqttOptions::new(
        &uuid,
        &env::var("BROKER_URL").unwrap_or(DEFAULT_BROKER.into()),
        1883,
    );
    mqtt_options.set_credentials(MQTT_USER, MQTT_PW);
    mqtt_options.set_keep_alive(400);
    println!("Connecting on: {}", mqtt_options.broker_address().0);

    let (client, mut eventloop) = AsyncClient::new(mqtt_options.clone(), 10);

    println!("Fetching registry");
    let registry = Arc::new(RwLock::new({
        let registry = mqtt_request::<Registry>(
            &format!("client/{}/reply/store/dentist/public/registry", uuid),
            &format!("store/dentist/public/{}/registry", uuid),
            &client,
            &mut eventloop,
        )
        .await
        .ok_or("Failed to fetch dentist registry")?;
        AvailOpeningHours::from(registry).ok_or("Failed to convert registry")?
    }));
    println!("Registry fetched");
    println!("{:?}", *registry.read().await);

    client.subscribe(MQTT_SUB_TOPIC, QoS::AtLeastOnce).await?;
    client.subscribe(DENTIST_TOPIC, QoS::AtLeastOnce).await?;

    let cl = Arc::new(client);
    let db = Arc::new(Mutex::new(conn));

    let inflight = Arc::new(AtomicI64::new(0));
    let inflight_limit: i64 = match env::var("INFLIGHT_LIMIT") {
        Ok(var) => var.parse().unwrap_or(INFLIGHT_LIMIT_DEFAULT),
        Err(_) => INFLIGHT_LIMIT_DEFAULT,
    };

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) if p.topic == DENTIST_TOPIC => {
                // We have to acquire the registry lock to write to it,
                // Since we are in the middle of the eventloop we spawn it with tokio to avoid blocking
                let reg = registry.clone();
                tokio::spawn(async move {
                    *reg.write().await =
                        match deserialize::<Registry>(SerialType::Cbor, p.payload.to_vec()) {
                            Some(e) => match AvailOpeningHours::from(e) {
                                Some(s) => s,
                                None => return,
                            },
                            _ => return,
                        };
                });
            }
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                if inflight.load(Ordering::SeqCst) < inflight_limit {
                    let (inflight, cl, db, registry) =
                        (inflight.clone(), cl.clone(), db.clone(), registry.clone());
                    tokio::spawn(async move {
                        inflight.fetch_add(1, Ordering::SeqCst);
                        handling::publish::handle_pub(p, cl, db, registry).await;
                        inflight.fetch_sub(1, Ordering::SeqCst);
                    });
                } else {
                    println!("Inflight limit reached!");
                }
            }
            Err(e) => println!("{:?}", e),
            _ => {}
        }
    }
}
