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
    let mut month_summary: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for item in history.operations.operations.iter() {
        for rule in rules.rules.iter() {
            if rule.match_rules.iter().any(|q| q.transaction_description_compiled.is_match(&item.description)) {
                let date_summary_key = item.order_date.format("%Y-%m").to_string();
                month_summary.entry(date_summary_key.clone()).or_insert(HashMap::new());

                month_summary
                    .entry(date_summary_key.clone())
                    .and_modify(|e| { e.entry(rule.category_name.clone()).or_insert(0.0); });

                month_summary
                    .entry(date_summary_key.clone())
                    .and_modify(|e| { e.entry(rule.category_name.clone()).and_modify(|e| *e += item.amount); });
            }
        }
    }

    println!("Account: {}", history.search.account);
    println!("Since: {} To: {}", history.search.date.since, history.search.date.to);

    let mut month_keys : Vec<_>= month_summary.keys().collect();
    month_keys.sort();

    for month in month_keys {
        println!("\n{}\n", *month);

        let mut cat_keys : Vec<_>= month_summary.get(month).unwrap().keys().collect();
        cat_keys.sort();

        for cat in cat_keys {
            let cat_amount = month_summary.get(month).unwrap().get(cat).unwrap();
            println!("{}: {}", cat, format!("{:.2}", cat_amount.abs()));
        }
    }
}
