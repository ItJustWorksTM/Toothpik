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

pub mod macros;
pub mod request;
pub mod structs;

pub async fn has_table(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, name: &str) -> bool {
    let result = sqlx::query(
        format!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
            name
        )
        .as_str(),
    )
    .fetch_optional(&mut *conn)
    .await;
    match result {
        Ok(ret) => ret.is_some(),
        Err(_) => false,
    }
}

pub static RE_UUID: &str =
    "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89ABab][0-9a-fA-F]{3}-[0-9a-fA-F]{12}";
