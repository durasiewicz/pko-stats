use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

const FORMAT: &'static str = "%Y-%m-%d";

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error> where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}