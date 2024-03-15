use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MatchRule {
    pub transaction_type: String,
    pub transaction_description: String,
}

#[derive(Deserialize)]
pub struct CategoryRule {
    pub category_name: String,
    pub match_rules: Vec<MatchRule>,
}

#[derive(Deserialize)]
pub struct CategoryRules {
    pub rules: Vec<CategoryRule>,
    pub ignore: Vec<CategoryRule>,
}

pub fn read_rules(file_path: &PathBuf) -> CategoryRules {
    let mut file = File::open(file_path).unwrap();
    return serde_json::from_reader(&file).unwrap();
}

