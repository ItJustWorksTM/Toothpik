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

use serde::{Deserialize, Serialize};

use super::user::UserInternal;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserAuthInc {
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub user_id: String,
    pub secret: String,
}

impl UserAuth {
    pub fn from_user(user: &UserInternal) -> UserAuth {
        UserAuth {
            user_id: user.id.clone(),
            username: user.username.clone(),
            secret: user.secret.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserAuthErr {
    pub username: String,
    pub error: (),
}
