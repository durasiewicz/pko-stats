use crate::category_rules::get_rules;

mod category_rules;

fn main() {
    let rules = get_rules("rules.json");

    println!("Hello, world!");
}
