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

use std::sync::Arc;

use rumqttc::{AsyncClient, QoS};
use store_utils::request::{serialize, Request, SerialType};
use store_utils::structs::Registry;
use tokio::sync::RwLock;
use tokio::time::Duration;

pub async fn fetch_registry(url: &str) -> Option<Registry> {
    match reqwest::get(url).await {
        Ok(res) => match res.json::<Registry>().await {
            Ok(regis) => Some(regis),
            Err(e) => {
                println!("failed to deserialize dentist registry:\n{}", e);
                None
            }
        },
        Err(_) => None,
    }
}

pub fn spawn_fetcher(
    registry: Arc<RwLock<Registry>>,
    client: Arc<AsyncClient>,
    url: String,
    interval: Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            println!("fetching registry...");
            match fetch_registry(&url).await {
                Some(reg) => {
                    let mut cur_reg = registry.read().await.clone();
                    if cur_reg == reg {
                        println!("Registry still the same");
                        // Simply continuing would re-trigger the check without waiting
                        // Since we want to not wait the first time the store starts up
                        // and rust not having do-while loops its best to just have a 2nd wait call
                        tokio::time::delay_for(interval).await;
                        continue;
                    } else {
                        let mut lk = registry.write().await;
                        *lk = reg;
                        cur_reg = lk.clone();
                    }

                    if match serialize(SerialType::Cbor, &cur_reg) {
                        Some(bin) => client
                            .publish(
                                "store/dentist/public/realtime/registry",
                                QoS::AtLeastOnce,
                                false,
                                &*bin,
                            )
                            .await
                            .ok(),
                        _ => None,
                    }
                    .is_some()
                    {
                        println!("Published updated registry.");
                    } else {
                        println!("Updated registry but could not publish");
                    }
                }
                None => println!("Could not reach server"),
            };
            tokio::time::delay_for(interval).await;
        }
    })
}

pub async fn request(
    req: Request<()>,
    client: Arc<AsyncClient>,
    registry: Arc<RwLock<Registry>>,
) -> Option<()> {
    let reg_lock = registry.read().await;
    client
        .publish(
            req.reply_topic,
            QoS::AtLeastOnce,
            false,
            serialize(req.serial_type, &*reg_lock)?,
        )
        .await
        .ok()
}
