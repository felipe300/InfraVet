use anyhow::Result;

use crate::{
    models::{RuleDefinition, RulesConfig},
    utils::output,
};

const RULES_JSON: &str = include_str!("../../docs/rules.json");

fn get_rules() -> Result<Vec<RuleDefinition>> {
    let config: RulesConfig = serde_json::from_str(RULES_JSON)?;

    Ok(config.rules)
}

pub(crate) fn rules() -> Result<()> {
    let rules = get_rules()?;

    output::rules(&rules);

    Ok(())
}
