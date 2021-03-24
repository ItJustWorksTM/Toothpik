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

use std::collections::HashMap;
use std::sync::Arc;

use rumqttc::{AsyncClient, QoS};
use rumqttc::{Event, EventLoop, Incoming, SubscribeReturnCodes};

use super::structs::Error;
use super::RE_UUID;

#[derive(Debug)]
pub struct TopicCaptures {
    pub topic: String,
    pub params: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug)]
pub enum SerialType {
    Json,
    Cbor,
}

pub fn serialize<S: serde::Serialize>(serial_type: SerialType, obj: &S) -> Option<Vec<u8>> {
    match serial_type {
        SerialType::Json => serde_json::to_vec(obj).ok(),
        SerialType::Cbor => serde_cbor::to_vec(obj).ok(),
    }
}

pub fn deserialize<S: serde::de::DeserializeOwned>(
    serial_type: SerialType,
    bytes: Vec<u8>,
) -> Option<S> {
    match serial_type {
        SerialType::Json => serde_json::from_slice(&bytes).ok(),
        SerialType::Cbor => serde_cbor::from_slice(&bytes).ok(),
    }
}

#[derive(Debug)]
pub struct Request<T: serde::de::DeserializeOwned> {
    pub serial_type: SerialType,
    pub topic: String,
    pub reply_topic: String,
    pub id: String,
    pub obj: Option<T>,
    pub caps: TopicCaptures,
}

impl<T: serde::de::DeserializeOwned> Request<T> {
    pub fn new(prefix: &str, bytes: Vec<u8>, caps: TopicCaptures) -> Option<Request<T>> {
        if let (Some(lhs), Some(id), Some(rhs)) = (
            caps.params.get("lhs"),
            caps.params.get("id"),
            caps.params.get("rhs"),
        ) {
            // Questionable way of checking if its json
            let serial_type = match bytes.first() {
                Some(t) if *t == 123u8 => SerialType::Json,
                _ => SerialType::Cbor,
            };

            Some(Request {
                serial_type,
                topic: format!("{}/{}/{}", lhs, id, rhs),
                reply_topic: format!("{}/{}/reply/{}/{}", prefix, id, lhs, rhs),
                id: id.clone(),
                obj: deserialize(serial_type, bytes),
                caps,
            })
        } else {
            None
        }
    }
}

pub fn mktopic(lhs: &str, rhs: &str) -> String {
    format!("(?P<lhs>{})/(?P<id>{})/(?P<rhs>{})", lhs, RE_UUID, rhs)
}

impl TopicCaptures {
    pub fn new(re: &regex::Regex, text: &str) -> Option<TopicCaptures> {
        let caps = re.captures(text)?;
        Some(TopicCaptures {
            topic: text.into(),
            params: re
                .capture_names()
                .flatten()
                .filter_map(|n| Some((String::from(n), String::from(caps.name(n)?.as_str()))))
                .collect(),
        })
    }
}

pub async fn send_err(
    topic: &str,
    msg: &str,
    serial_type: SerialType,
    client: Arc<AsyncClient>,
) -> Option<()> {
    client
        .publish(
            topic,
            QoS::AtLeastOnce,
            false,
            serialize(serial_type, &Error::new(msg))?,
        )
        .await
        .ok()?;
    Some(())
}

pub async fn mqtt_request<T: serde::de::DeserializeOwned>(
    rtc: &str,
    itc: &str,
    client: &AsyncClient,
    ev: &mut EventLoop,
) -> Option<T> {
    client.subscribe(rtc, QoS::AtLeastOnce).await.ok()?;
    loop {
        match ev.poll().await.ok()? {
            Event::Incoming(Incoming::Publish(p)) if p.topic == rtc => {
                client.unsubscribe(rtc).await.ok()?;
                return serde_cbor::from_slice(&p.payload).ok()?;
            }
            Event::Incoming(Incoming::SubAck(ack)) => {
                if ack.return_codes.contains(&SubscribeReturnCodes::Failure) {
                    return None;
                }
                client.publish(itc, QoS::AtMostOnce, false, []).await.ok()?;
            }
            _ => {}
        };
    }
}
