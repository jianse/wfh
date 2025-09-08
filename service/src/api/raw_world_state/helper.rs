use std::num::ParseIntError;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

pub fn bool_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Clone)]
pub struct Oid {
    #[serde(rename = "$oid")]
    pub oid: String,
}

impl From<Oid> for String {
    fn from(oid: Oid) -> Self {
        oid.oid
    }
}
impl From<String> for Oid {
    fn from(oid: String) -> Self {
        Oid { oid }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date {
    #[serde(rename = "$date")]
    pub date: DateNumber,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DateNumber {
    #[serde(rename = "$numberLong")]
    pub number_long: String,
}

#[derive(Debug, Error)]
pub enum ParseDateError {
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("invalid timestamp")]
    TimeStamp,
}

impl TryFrom<Date> for DateTime<Utc> {
    type Error = ParseDateError;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
        let number_long = date.date.number_long;
        let number_long = number_long.parse::<i64>()?;
        let date = DateTime::from_timestamp_millis(number_long);
        match date {
            Some(d) => Ok(d),
            None => Err(ParseDateError::TimeStamp),
        }
    }
}
