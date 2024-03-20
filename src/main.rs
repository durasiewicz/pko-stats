use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use tabled::builder::Builder;
use tabled::settings::{Modify, Width};
use tabled::settings::object::Segment;
use tabled::Table;

use crate::account_history::{Operation, read_history};
use crate::category_rules::{Matching, read_rules};

mod category_rules;
mod account_history;
mod date_deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    account_history: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    category_rules: Option<PathBuf>,

    #[arg(short, long, action)]
    show_uncategorized: bool,
}

fn main() {
    let args = Args::parse();

    let category_rules = args.category_rules.unwrap_or_else(|| PathBuf::from("rules.json"));
    let rules = read_rules(&category_rules);
    let history = read_history(&args.account_history);
    let mut month_summary: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut uncategorized: Vec<Operation> = Vec::new();

    for item in history.operations.operations.into_iter() {
        let mut has_matched_category = false;

        for rule in rules.rules.iter() {
            if rule.is_match(&item.op_type, &item.description) {
                let date_summary_key = item.order_date.format("%Y-%m").to_string();
                month_summary.entry(date_summary_key.clone()).or_insert(HashMap::new());

                month_summary
                    .entry(date_summary_key.clone())
                    .and_modify(|e| { e.entry(rule.category_name.clone()).or_insert(0.0); });

                month_summary
                    .entry(date_summary_key.clone())
                    .and_modify(|e| { e.entry(rule.category_name.clone()).and_modify(|e| *e += item.amount); });

                has_matched_category = true;
            }
        }

        if args.show_uncategorized && !has_matched_category {
            uncategorized.push(item);
        }
    }

    println!("Account: {}", history.search.account);
    println!("Since: {} To: {}", history.search.date.since, history.search.date.to);

    let mut month_keys: Vec<_> = month_summary.keys().collect();
    month_keys.sort();

    for month in month_keys {
        println!("\n{}\n", *month);

        let mut cat_keys: Vec<_> = month_summary.get(month).unwrap().keys().collect();
        cat_keys.sort();

        let mut table_builder = Builder::default();
        let mut amount_summary = 0.0;

        for cat in cat_keys {
            let cat_amount = month_summary.get(month).unwrap().get(cat).unwrap();
            table_builder.push_record(vec![cat, &format!("{:.2}", cat_amount)]);
            amount_summary += cat_amount;
        }

        table_builder.push_record(vec!["", &format!("{:.2}", amount_summary)]);

        let table = table_builder.build();
        println!("{}", table.to_string());
    }

    if !uncategorized.is_empty() {
        println!("\nUncategorized operations:\n");

        let mut table = Table::new(uncategorized);
        table.with(Modify::new(Segment::all()).with(Width::wrap(50)));

        println!("{}", table.to_string());
    }
}
