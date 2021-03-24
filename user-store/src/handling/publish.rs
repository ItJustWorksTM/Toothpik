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

use handlers::*;

use super::*;
#[cfg(feature = "mail_check")]
use lettre::SmtpTransport;
use sqlx::pool::PoolConnection;
use tokio::sync::Mutex;

pub async fn handle_pub(
    pu: Publish,
    cl: Arc<AsyncClient>,
    db: Arc<Mutex<PoolConnection<Sqlite>>>,
    #[cfg(feature = "mail_check")] ml: Arc<Mutex<SmtpTransport>>,
) -> Option<()> {
    static_def_regex! {
        regexs, // the vec
        secret: &mktopic("store/user", "secret"),
        features: &mktopic("store/user/public", "features"),
        reg_user: &mktopic("store/user/public", "register"),
        val_user: &mktopic("store/user/public", "validate"),
        get_user: &mktopic("store/user", "self"),
        user_id: &mktopic("store/user", "my_id")
    }

    for i in 0..regexs.len() {
        match TopicCaptures::new(&regexs[i], pu.topic.as_str()) {
            Some(cap) => {
                let load = pu.payload.to_vec();
                let clc = cl.clone();
                let dbc = db.clone();
                #[cfg(feature = "mail_check")]
                let mlc = ml.clone();

                macro_rules! callv_dbcl {
                    ($prefix: expr, $func: path) => {
                        $func(Request::new($prefix, load, cap)?,
                            clc,
                            dbc,
                        ).await
                    };
                    ($prefix: expr, $func: path, $($extra_args:expr),*) => {
                        $func(Request::new($prefix, load, cap)?,
                            clc,
                            dbc,
                            $($extra_args),*
                        ).await
                    };
                }

                let req = |a: &Regex| std::ptr::eq(a, *regexs[i]);
                match match *regexs[i] {
                    _ if req(secret) => callv_dbcl!("auth", auth::inc),
                    _ if req(features) => callv_dbcl!("client", user::features),
                    _ if req(reg_user) => callv_dbcl!(
                        "client",
                        user::register,
                        #[cfg(feature = "mail_check")]
                        mlc
                    ),
                    #[cfg(feature = "mail_check")]
                    _ if req(val_user) => callv_dbcl!("client", user::validate),
                    _ if req(get_user) => callv_dbcl!("client", user::get_self),
                    _ if req(user_id) => callv_dbcl!("client", user::get_id),
                    _ => None,
                } {
                    // For now we use empty optionals to make use of ? syntax,
                    // in the future we might want to look at Result so that we can convey what failed.
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
