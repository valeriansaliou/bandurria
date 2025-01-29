// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use chrono::offset::Utc;
use chrono::NaiveDateTime;

const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

const DATETIME_TO_DATE_FORMAT: &'static str = "%d/%m/%Y";
const DATETIME_TO_TIME_FORMAT: &'static str = "%Hh%M";
const DATETIME_TO_UTC_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

const FALLBACK_DATETIME_STRING: &'static str = "(?)";

pub fn now_datetime_string() -> String {
    Utc::now().format(DATETIME_FORMAT).to_string()
}

pub fn now_after_datetime_string(after: Duration) -> String {
    (Utc::now() + after).format(DATETIME_FORMAT).to_string()
}

pub fn parse_datetime_string(datetime: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(datetime, DATETIME_FORMAT)
        .map_err(|err| {
            error!(
                "could not parse datatime string: {} because: {}",
                datetime, err
            );
        })
        .ok()
}

pub fn datetime_to_string(datetime: &Option<NaiveDateTime>, format: &str) -> String {
    datetime
        .map(|datetime| datetime.format(format).to_string())
        .unwrap_or(FALLBACK_DATETIME_STRING.to_string())
}

pub fn datetime_to_date_string(datetime: &Option<NaiveDateTime>) -> String {
    datetime_to_string(datetime, DATETIME_TO_DATE_FORMAT)
}

pub fn datetime_to_time_string(datetime: &Option<NaiveDateTime>) -> String {
    datetime_to_string(datetime, DATETIME_TO_TIME_FORMAT)
}

pub fn datetime_to_utc_string(datetime: &Option<NaiveDateTime>) -> String {
    datetime_to_string(datetime, DATETIME_TO_UTC_FORMAT)
}
