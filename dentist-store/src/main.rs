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
use std::sync::atomic::{AtomicI64, Ordering};
use store_utils::structs::Registry;
use tokio::sync::RwLock;
use tokio::time::Duration;
use uuid::Uuid;

use definitions::*;

use crate::handling::publish::handle_pub;

mod definitions;
mod handling;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqtt_options = MqttOptions::new(
        format!("{}", Uuid::new_v4()),
        &env::var("BROKER_URL").unwrap_or(MQTT_DEFAULT.into()),
        1883,
    );
    mqtt_options.set_credentials(MQTT_USER, MQTT_PW);
    mqtt_options.set_keep_alive(400);

    println!("Connecting on: {}", mqtt_options.broker_address().0);

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe(MQTT_SUB_TOPIC, QoS::AtLeastOnce).await?;

    let registry = Arc::new(RwLock::new(Registry {
        dentists: Vec::default(),
    }));

    let cl = Arc::new(client);
    handling::handlers::registry::spawn_fetcher(
        registry.clone(),
        cl.clone(),
        REGISTRY_URL.into(),
        Duration::from_secs(30),
    );

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
                        let (inflight, cl, registry) =
                            (inflight.clone(), cl.clone(), registry.clone());
                        tokio::spawn(async move {
                            inflight.fetch_add(1, Ordering::SeqCst);
                            handle_pub(p, cl, registry).await;
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
