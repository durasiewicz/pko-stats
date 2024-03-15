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
    for item in history.operations.operations.iter() {
        println!("{}", item.description);
    }

    println!("Hello, world!");
}
