use std::fs::File;
use std::path::PathBuf;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::date_deserialize;

#[derive(Debug, Deserialize)]
pub struct DateRange {
    #[serde(rename = "since", with = "date_deserialize")]
    pub since: NaiveDate,
    #[serde(rename = "to", with = "date_deserialize")]
    pub to: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub account: String,
    pub date: DateRange,
    pub filtering: String,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    #[serde(rename = "order-date", with = "date_deserialize")]
    pub order_date: NaiveDate,
    #[serde(rename = "exec-date", with = "date_deserialize")]
    pub exec_date: NaiveDate,
    #[serde(rename = "type")]
    pub op_type: String,
    pub description: String,
    pub amount: f64,
    #[serde(rename = "ending-balance")]
    pub ending_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct Amount {
    #[serde(rename = "curr", default)]
    pub currency: String,
    #[serde(rename = "$value", default)]
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct Operations {
    #[serde(rename = "operation", default)]
    pub operations: Vec<Operation>,
}

#[derive(Debug, Deserialize)]
pub struct AccountHistory {
    pub search: Search,
    pub operations: Operations,
}

pub fn read_history(file_path: &PathBuf) -> AccountHistory {
    let file = File::open(file_path).unwrap();
    return serde_xml_rs::from_reader(&file).unwrap();
}