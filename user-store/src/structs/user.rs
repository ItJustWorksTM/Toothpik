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
use sqlx::FromRow;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct UserInternal {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub secret: String,
    pub verification_code: i32,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub secret: String,
    #[cfg(feature = "reg_captcha")]
    pub captcha_token: String,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct UserValidation {
    pub username: String,
    pub verification_code: i32,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct UserId {
    pub id: String,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct UserName {
    pub username: String,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
}
