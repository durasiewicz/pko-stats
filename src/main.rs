use std::collections::HashMap;
use std::path::PathBuf;
use clap::Parser;
use crate::account_history::read_history;
use crate::category_rules::read_rules;

mod category_rules;
mod account_history;
mod date_deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    account_history: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    category_rules: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let category_rules = args.category_rules.unwrap_or_else(|| PathBuf::from("rules.json"));
    let rules = read_rules(&category_rules);
    let history = read_history(&args.account_history);
    let mut categories_summary: HashMap<String, f64> = HashMap::new();

    for item in history.operations.operations.iter() {
        for rule in rules.rules.iter() {
            if rule.match_rules.iter().any(|q| q.transaction_description_compiled.is_match(&item.description)) {
                categories_summary.entry(rule.category_name.clone()).or_insert(0.0);
                categories_summary.entry(rule.category_name.clone()).and_modify(|e| *e += item.amount);
            }
        }
    }

    println!("Account: {}", history.search.account);
    println!("Since: {} To: {} \n", history.search.date.since, history.search.date.to);

    for entry in categories_summary.iter() {
        println!("{}: {}", entry.0, format!("{:.2}", entry.1.abs()));
    }
}
