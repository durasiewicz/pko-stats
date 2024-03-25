use std::fs::File;
use std::path::PathBuf;

use regex::{Regex, RegexBuilder};
use serde::Deserialize;

fn default_regex() -> Regex {
    Regex::new("").unwrap()
}

fn empty_string() -> String { "".to_string() }

#[derive(Deserialize)]
pub struct MatchRule {
    #[serde(default = "empty_string")]
    pub transaction_type: String,
    #[serde(skip, default = "default_regex")]
    pub transaction_type_compiled: Regex,
    #[serde(default = "empty_string")]
    pub transaction_description: String,
    #[serde(skip, default = "default_regex")]
    pub transaction_description_compiled: Regex,
}

#[derive(Deserialize)]
pub struct CategoryRule {
    #[serde(default = "empty_string")]
    pub category_name: String,
    pub match_rules: Vec<MatchRule>,
}

#[derive(Deserialize)]
pub struct CategoryRules {
    pub rules: Vec<CategoryRule>,
    pub ignore: Vec<MatchRule>,
}

fn compile_rule_match(mut rule_match: MatchRule) -> MatchRule {
    if !rule_match.transaction_type.is_empty() {
        rule_match.transaction_type_compiled = RegexBuilder::new(&rule_match.transaction_type)
            .case_insensitive(true)
            .build()
            .unwrap();
    }

    if !rule_match.transaction_description.is_empty() {
        rule_match.transaction_description_compiled = RegexBuilder::new(&rule_match.transaction_description)
            .case_insensitive(true)
            .build()
            .unwrap();
    }

    rule_match
}

pub trait Matching {
    fn is_match(&self, operation_type: &String, description: &String) -> bool;
}

impl Matching for MatchRule {
    fn is_match(&self, operation_type: &String, description: &String) -> bool {
        !self.transaction_type.is_empty() && self.transaction_type_compiled.is_match(&operation_type) ||
            !self.transaction_description.is_empty() && self.transaction_description_compiled.is_match(&description)
    }
}

impl Matching for CategoryRule {
    fn is_match(&self, operation_type: &String, description: &String) -> bool {
        self.match_rules.iter().any(|q| q.is_match(operation_type, description))
    }
}

pub fn read_rules(file_path: &PathBuf) -> CategoryRules {
    let file = File::open(file_path).unwrap();
    let content: CategoryRules = serde_json::from_reader(&file).unwrap();

    let result = CategoryRules
    {
        rules: content.rules.into_iter()
            .map(|mut x| {
                if x.category_name.is_empty() {
                    panic!("Matching rule category name can't be empty.")
                }

                x.match_rules = x.match_rules.into_iter().map(|item| compile_rule_match(item)).collect();
                return x;
            })
            .collect(),

        ignore: content.ignore.into_iter()
            .map(|x| compile_rule_match(x))
            .collect(),
    };

    result
}

