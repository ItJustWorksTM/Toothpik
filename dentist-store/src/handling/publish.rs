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

use regex::Regex;
use rumqttc::{AsyncClient, Publish};
use store_utils::request::Request;
use store_utils::request::{mktopic, TopicCaptures};
use store_utils::static_def_regex;
use store_utils::structs::Registry;
use tokio::sync::RwLock;

use super::handlers::registry;

pub async fn handle_pub(
    pu: Publish,
    cl: Arc<AsyncClient>,
    reg: Arc<RwLock<Registry>>,
) -> Option<()> {
    static_def_regex! {
        regexs, // the vec
        registry_request: &mktopic("store/dentist/public", "registry")
    }
    for i in 0..regexs.len() {
        match TopicCaptures::new(&regexs[i], pu.topic.as_str()) {
            Some(cap) => {
                let clc = cl.clone();
                let regc = reg.clone();
                macro_rules! callv_regcl {
                    ($prefix: expr, $func: path) => {
                        $func(Request::new($prefix, Vec::default(), cap)?, clc, regc).await
                    };
                }

                let req = |a: &Regex| std::ptr::eq(a, *regexs[i]);
                match match *regexs[i] {
                    _ if req(registry_request) => callv_regcl!("client", registry::request),
                    _ => None,
                } {
                    Some(_) => {}
                    None => println!("Failed to handle `{}`", pu.topic),
                }
                break;
            }
            None => (),
        }
    }
    Some(())
}
