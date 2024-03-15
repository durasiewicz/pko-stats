use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MatchRule {
    transaction_type: String,
    transaction_description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryRule {
    category_name: String,
    match_rules: Vec<MatchRule>,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryRules {
    rules: Vec<CategoryRule>,
    ignore: Vec<CategoryRule>,
}

pub fn get_rules(file_path: &str) -> CategoryRules {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return serde_json::from_str(&contents).unwrap();
}

