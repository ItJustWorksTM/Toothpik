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

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Generic error message
#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
}

impl Error {
    pub fn new(error: &str) -> Error {
        Error {
            error: error.into(),
        }
    }
}

// Dentist registry
#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct OpeningHours {
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
}

#[derive(Debug)]
pub struct AvailOpeningHours {
    pub id: i64,
    pub dentists: i64,
    pub openinghours: [(NaiveTime, NaiveTime); 5],
}

impl AvailOpeningHours {
    pub fn from(x: Registry) -> Option<Vec<AvailOpeningHours>> {
        let fmt = |a: &str| NaiveTime::parse_from_str(a, "%k:%M").ok();
        macro_rules! mk {
            ($mb:expr) => {{
                let split: Vec<&str> = $mb.split('-').collect();
                (fmt(&split.get(0)?)?, fmt(&split.get(1)?)?)
            }};
        };
        let mut ret = Vec::default();
        let gen = |t: OpeningHours| {
            Some([
                mk!(t.monday),
                mk!(t.tuesday),
                mk!(t.wednesday),
                mk!(t.thursday),
                mk!(t.friday),
            ])
        };
        for item in x.dentists {
            ret.push(AvailOpeningHours {
                id: item.id,
                dentists: item.dentists,
                openinghours: gen(item.openinghours)?,
            });
        }

        Some(ret)
    }
}

#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct RegistryItem {
    pub id: i64,
    pub name: String,
    pub owner: String,
    pub dentists: i64,
    pub address: String,
    pub city: String,
    pub coordinate: Coordinate,
    pub openinghours: OpeningHours,
}

#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Registry {
    pub dentists: Vec<RegistryItem>,
}

// Availability
#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Availability {
    pub dentistid: i64,
    pub availability: Vec<AvailabilityDay>,
}

#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AvailabilityDay {
    pub date: String,
    pub time: Vec<AvailabilityTime>,
}

#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AvailabilityTime {
    pub time: String,
    pub available: bool,
}

// Availability update
#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AvailabilityUpdate {
    pub dentistid: i64,
    pub date: String,
    pub time: String,
    pub available: bool,
}

// Availability request
#[derive(PartialOrd, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AvailabilityRequest {
    pub dentistid: i64,
    pub start_date: String,
    pub end_date: String,
}
