use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MatchRule {
    transaction_type: String,
    transaction_description: String,
}

#[derive(Serialize, Deserialize)]
struct CategoryRule {
    category_name: String,
    match_rules: Vec<MatchRule>,
}

#[derive(Serialize, Deserialize)]
struct Rules {
    rules: Vec<CategoryRule>,
    ignore: Vec<CategoryRule>,
}

fn main() {
    let mut file = File::open("rules.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let rules : Rules = serde_json::from_str(&contents).unwrap();

    println!("Hello, world!");
}
