use anyhow::Result;
// use serde::Deserialize;

use crate::models::{Rule, RulesConfig};

const RULES_JSON: &str = include_str!("../../docs/rules.json");

fn get_rules() -> Result<Vec<Rule>> {
    let config: RulesConfig = serde_json::from_str(RULES_JSON)?;

    Ok(config.rules)
}

pub (crate) fn rules() -> Result<()> {
    println!("Implemented Rules");

    for rule in get_rules()? {
        println!("{} [{}] - {}", rule.code, rule.severity, rule.title);
        println!("  Category: {}", rule.category);
        println!("  Description: {}", rule.description);
        println!("  Recommendation: {}", rule.recommendation);
        println!("  Example:");
        println!("{}.", rule.example.dockerfile);
        println!();
    }

    Ok(())
}
