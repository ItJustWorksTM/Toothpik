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

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::definitions::DATETIME_FORMAT;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct QAppointment {
    pub userid: String,
    pub requestid: String,
    pub dentistid: i64,
    pub issuance: i64,
    pub time: String,
}

impl QAppointment {
    pub fn sql(aptmnt: QAppointment) -> Option<SQLAppointment> {
        Some(SQLAppointment {
            userid: aptmnt.userid,
            dentistid: aptmnt.dentistid,
            date: NaiveDateTime::parse_from_str(&aptmnt.time, DATETIME_FORMAT).ok()?,
            reason: None,
            mobile: None,
            email: None,
            name: None,
        })
    }
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct NAppointment {
    pub userid: String,
    pub dentistid: i64,
    pub time: String,
    pub reason: String,
    pub mobile: String,
    pub email: String,
    pub name: String,
}

impl NAppointment {
    pub fn sql(aptmnt: NAppointment) -> Option<SQLAppointment> {
        Some(SQLAppointment {
            userid: aptmnt.userid,
            dentistid: aptmnt.dentistid,
            date: NaiveDateTime::parse_from_str(&aptmnt.time, DATETIME_FORMAT).ok()?,
            reason: aptmnt.reason.into(),
            mobile: aptmnt.mobile.into(),
            email: aptmnt.email.into(),
            name: aptmnt.name.into(),
        })
    }
}

// The 1:1 map to our sql format
#[derive(FromRow, Clone, Debug)]
pub struct SQLAppointment {
    pub userid: String,
    pub dentistid: i64,
    pub date: NaiveDateTime,
    pub reason: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct AppointmentResponse {
    pub userid: String,
    pub requestid: String,
    // for compatibility reasons
    pub time: String,
}
