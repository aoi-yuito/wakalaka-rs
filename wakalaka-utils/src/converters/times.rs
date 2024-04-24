// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Timestamp;
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};

pub fn datetime_to_naivedatetime(ts: &Timestamp) -> NaiveDateTime {
    DateTime::<Utc>::from_timestamp(ts.timestamp(), 0)
        .expect("Failed to convert DateTime to NaiveDateTime")
        .naive_utc()
}
