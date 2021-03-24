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

#[cfg(feature = "mail_check")]
use lettre::message::Mailbox;
#[cfg(feature = "mail_check")]
use once_cell::sync::Lazy;

pub static MQTT_USER: &str = "store-user-1";
pub static MQTT_PW: &str = "null";
pub static MQTT_SUB_TOPIC: &str = "store/user/#";
pub static MQTT_DEFAULT: &str = "aerostun.dev";

pub static INFLIGHT_LIMIT_DEFAULT: i64 = 10;

#[cfg(feature = "reg_captcha")]
pub static CAPTCHA_VERIFY_URL: &str = "https://hcaptcha.com/siteverify";
#[cfg(feature = "reg_captcha")]
pub static CAPTCHA_DEFAULT_SECRET: &str = "0x0000000000000000000000000000000000000000";

#[cfg(feature = "mail_check")]
pub static SMTP_RELAY_DOMAIN: &str = "mail.domain.ltd";
#[cfg(feature = "mail_check")]
pub static SMTP_USER: &str = "mailer";
#[cfg(feature = "mail_check")]
pub static SMTP_PW: &str = "mailer-pw";
#[cfg(feature = "mail_check")]
pub static SMTP_SENDER: Lazy<Mailbox> = Lazy::new(|| {
    "ToothpikTM User Management <noreply+toothpik@aerostun.dev>"
        .parse()
        .unwrap()
});
