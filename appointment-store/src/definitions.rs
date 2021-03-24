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

pub static MQTT_USER: &str = "store-appointment-1";
pub static MQTT_PW: &str = "null";
pub static MQTT_SUB_TOPIC: &str = "store/appointment/#";
pub static DENTIST_TOPIC: &str = "store/dentist/public/realtime/registry";
pub static DEFAULT_BROKER: &str = "localhost";
pub static DATETIME_FORMAT: &str = "%F %H:%M";
pub static DATE_FORMAT: &str = "%F";
pub static TIME_FORMAT: &str = "%H:%M";
pub static INFLIGHT_LIMIT_DEFAULT: i64 = 10;
