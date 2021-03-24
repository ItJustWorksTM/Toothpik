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

use crate::structs::auth::*;
use crate::structs::UserInternal;

use super::*;
use sqlx::pool::PoolConnection;
use tokio::sync::Mutex;

pub async fn inc(
    req: Request<UserAuthInc>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
) -> Option<()> {
    match req.obj {
        Some(obj) => {
            let user = {
                sqlx::query_as::<_, UserInternal>("SELECT * FROM users WHERE username = ?")
                    .bind(&obj.username)
                    .fetch_one(&mut *data.lock().await)
                    .await
            };
            client
                .publish(
                    req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    match user {
                        Ok(user) if user.verification_code == 0 => {
                            let ret = UserAuth::from_user(&user);
                            serde_cbor::to_vec(&ret)
                        }
                        _ => serde_cbor::to_vec(&UserAuthErr {
                            username: obj.username.clone(),
                            error: (),
                        }),
                    }
                    .ok()?,
                )
                .await
                .ok()?;
        }
        None => send_err(&req.reply_topic, "Invalid Format", client).await?,
    }
    Some(())
}
