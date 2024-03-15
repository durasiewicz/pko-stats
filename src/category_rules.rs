use std::fs::File;
use std::path::PathBuf;

use regex::{Regex, RegexBuilder};
use serde::Deserialize;

fn default_regex() -> Regex {
    Regex::new("").unwrap()
}

#[derive(Deserialize)]
pub struct MatchRule {
    pub transaction_type: String,
    #[serde(skip, default = "default_regex")]
    pub transaction_type_compiled: Regex,

    pub transaction_description: String,
    #[serde(skip, default = "default_regex")]
    pub transaction_description_compiled: Regex,
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

    return rule_match;
}

pub fn read_rules(file_path: &PathBuf) -> CategoryRules {
    let file = File::open(file_path).unwrap();
    let content: CategoryRules = serde_json::from_reader(&file).unwrap();

    let result = CategoryRules
    {
        rules: content.rules.into_iter()
            .map(|mut x| {
                x.match_rules = x.match_rules.into_iter().map(|item| compile_rule_match(item)).collect();
                return x;
            })
            .collect(),

        ignore: content.ignore.into_iter()
            .map(|mut x| {
                x.match_rules = x.match_rules.into_iter().map(|item| compile_rule_match(item)).collect();
                return x;
            })
            .collect(),
    };

    return result;
}

