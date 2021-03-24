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

use chrono;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use store_utils::structs::*;
use tokio::sync::RwLock;

use crate::definitions::{DATETIME_FORMAT, DATE_FORMAT, TIME_FORMAT};

use super::*;

#[derive(sqlx::FromRow, Debug)]
struct IdDate {
    pub id: String,
    pub date: NaiveDateTime,
}

pub async fn is_available(
    dentistid: i64,
    date_time: NaiveDateTime,
    registry: &Vec<AvailOpeningHours>,
    data: &mut PoolConnection<Sqlite>,
) -> Option<AvailabilityUpdate> {
    let dentist = { registry.iter().find(|d| d.id == dentistid)? };

    let x = dentist
        .openinghours
        .get((date_time.weekday().number_from_monday() - 1) as usize)?;
    let mut pin = NaiveTime::from_hms(0, 0, 0);
    let time = date_time.time();
    let date = date_time.date();

    // Check if the date is valid in our slot systems
    // And then get the slot range for use in the SQL query
    let range = (|| {
        for j in 0..48 {
            let next = pin + Duration::minutes(30);
            if pin >= x.0
                && pin <= x.1
                && j != 25
                && j != 26
                && j != 22
                && time >= pin
                && time < next
            {
                return Some((date.and_time(pin), date.and_time(next)));
            }
            pin = next;
        }
        None
    })()?;

    // Fetch all
    let query = format!(
        "select id, date from appointments where dentistid = {} and date >= '{}' and date < '{}';",
        dentistid,
        range.0.format(DATETIME_FORMAT).to_string(),
        range.1.format(DATETIME_FORMAT).to_string()
    );

    // if the amount of appointments in said range is larger
    // than the total dentists that slot is not available
    // TODO: find more efficient way to count
    let appointments = sqlx::query_as::<_, IdDate>(&query)
        .fetch_all(data)
        .await
        .ok()?
        .len();
    if appointments < dentist.dentists as usize {
        // We generate an availability update that could be used if a new appointment is made
        // available field is calculated based on the knowledge that one more appointment
        // would make the slot unavailable
        return Some(AvailabilityUpdate {
            dentistid,
            date: range.0.format(DATE_FORMAT).to_string(),
            time: range.0.format(TIME_FORMAT).to_string(),
            available: appointments + 1 < dentist.dentists as usize,
        });
    }
    None
}

pub async fn get_avail(
    req: Request<AvailabilityRequest>,
    client: Arc<AsyncClient>,
    data: Arc<Mutex<PoolConnection<Sqlite>>>,
    registry: Arc<RwLock<Vec<AvailOpeningHours>>>,
) -> Option<()> {
    match req.obj {
        Some(obj) => {
            let (start, end) = match (
                NaiveDate::parse_from_str(&obj.start_date, DATE_FORMAT),
                NaiveDate::parse_from_str(&obj.end_date, DATE_FORMAT),
            ) {
                (Ok(start), Ok(end)) => (start, end),
                _ => {
                    return send_err(
                        &req.reply_topic,
                        "Invalid date format",
                        req.serial_type,
                        client,
                    )
                    .await
                }
            };

            let registry = registry.read().await;
            let dentist = {
                match registry.iter().find(|d| d.id == obj.dentistid) {
                    Some(dentist) => dentist,
                    None => {
                        return send_err(
                            &req.reply_topic,
                            "Dentist doesn't exist",
                            req.serial_type,
                            client,
                        )
                        .await
                    }
                }
            };

            // get all appointments on request period for specific dentist
            let real: Vec<IdDate> = {
                let mut data_lk = data.lock().await;
                let query = format!(
                    "select id, date from appointments where dentistid = {} and date >= '{}' and date < '{}';",
                    obj.dentistid,
                    start.format(DATE_FORMAT).to_string(),
                    (end+Duration::days(1)).format(DATE_FORMAT).to_string()
                );
                sqlx::query_as::<_, IdDate>(&query)
                    .fetch_all(&mut *data_lk)
                    .await
                    .ok()?
            };

            let mut avail = Availability {
                dentistid: obj.dentistid,
                availability: Vec::default(),
            };

            if start > end {
                return send_err(
                    &req.reply_topic,
                    "Start date larger than end date",
                    req.serial_type,
                    client,
                )
                .await;
            }

            let range = end - start + Duration::days(1);

            if range.num_days() > 32 {
                return send_err(
                    &req.reply_topic,
                    "Requesting availability of more than a month is forbidden",
                    req.serial_type,
                    client,
                )
                .await;
            }

            for i in 0..range.num_days() {
                let today = start + Duration::days(i);

                let day_avail = {
                    avail.availability.push(AvailabilityDay {
                        date: today.format(DATE_FORMAT).to_string(),
                        time: Vec::default(),
                    });
                    avail.availability.last_mut()?
                };

                let aptmnts: Vec<&IdDate> =
                    real.iter().filter(|t| t.date.date() == today).collect();

                let week_day =
                    ((start + Duration::days(i)).weekday().number_from_monday() - 1) as usize;

                // Non workdays can't be considered as we dont have any data on it
                if week_day > 4 {
                    continue;
                }

                let x = dentist.openinghours.get(week_day)?;
                let mut pin = NaiveTime::from_hms(0, 0, 0);
                for j in 0..48 {
                    let next = pin + Duration::minutes(30);
                    // We want the timeslots in the bounds of the opening hours
                    // And exclude lunch / break times, since they are 1h and 30 min respectfully
                    // Its easier to just check the timeslot of the day
                    if pin >= x.0 && pin <= x.1 && j != 25 && j != 26 && j != 22 {
                        day_avail.time.push(AvailabilityTime {
                            time: pin.format(TIME_FORMAT).to_string(),
                            available: if !aptmnts.is_empty() {
                                aptmnts
                                    .iter()
                                    .filter(|t| {
                                        // Goal is to see if there are appointments within the range of ´pin´ and ´pin + 30min`
                                        // Comparing the amount of dentists with the amount of appointments on that slot
                                        // Gives us the availability status
                                        let time = t.date.time();
                                        time >= pin && time < next
                                    })
                                    .count()
                                    < dentist.dentists as usize
                            } else {
                                true
                            },
                        });
                    }
                    pin = next;
                }
            }

            client
                .publish(
                    req.reply_topic,
                    QoS::AtLeastOnce,
                    false,
                    &*serialize(req.serial_type, &avail)?,
                )
                .await
                .ok()?;
        }
        None => {
            send_err(
                &req.reply_topic,
                "Invalid request format",
                req.serial_type,
                client,
            )
            .await?
        }
    };
    Some(())
}
