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

use regex::Regex;
use store_utils::request::{mktopic, TopicCaptures};
use store_utils::static_def_regex;
use store_utils::structs::AvailOpeningHours;
use tokio::sync::RwLock;

use handlers::*;

use super::*;

pub async fn handle_pub(
    pu: Publish,
    cl: Arc<AsyncClient>,
    db: Arc<Mutex<PoolConnection<Sqlite>>>,
    reg: Arc<RwLock<Vec<AvailOpeningHours>>>,
) -> Option<()> {
    static_def_regex! {
        regexs, // the vec
        qbook: &mktopic("store/appointment", "quick_book"),
        book: &mktopic("store/appointment", "book"),
        aptms: &mktopic("store/appointment", ""),
        avail: &mktopic("store/appointment/public", "availability")
    }

    for i in 0..regexs.len() {
        match TopicCaptures::new(&regexs[i], pu.topic.as_str()) {
            Some(cap) => {
                let load = pu.payload.to_vec();
                let clc = cl.clone();
                let dbc = db.clone();

                macro_rules! callv_dbcl {
                    ($prefix: expr, $func: path) => {
                        $func(Request::new($prefix, load, cap)?, clc, dbc, reg).await
                    };
                }
                let req = |a: &Regex| std::ptr::eq(a, *regexs[i]);
                match match *regexs[i] {
                    _ if req(qbook) => callv_dbcl!("client", appointment::qbook),
                    _ if req(book) => callv_dbcl!("client", appointment::nbook),
                    _ if req(avail) => callv_dbcl!("client", availability::get_avail),
                    _ => {
                        println!("Unhandled topic: {}", pu.topic);
                        Some(())
                    }
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
