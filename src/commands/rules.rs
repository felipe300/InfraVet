use anyhow::Result;

use crate::{
    models::{RuleDefinition, RulesConfig},
    reports::output,
};

const RULES_JSON: &str = include_str!("../../docs/rules.json");

fn get_rules() -> Result<Vec<RuleDefinition>> {
    let config: RulesConfig = serde_json::from_str(RULES_JSON)?;

    Ok(config.rules)
}

pub fn rules() -> Result<()> {
    let rules = get_rules()?;

    output::rules(&rules);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_rules_json_loads_successfully() {
        let rules_result = get_rules();
        assert!(
            rules_result.is_ok(),
            "Error to desarialize rules.json: {:?}",
            rules_result.err()
        );

        let rules = rules_result.unwrap();
        assert!(!rules.is_empty(), "rules.json should not be empty");
    }

    #[test]
    fn test_rules_have_unique_code_and_valid_fields() {
        let rules = get_rules().unwrap();
        let mut codes = HashSet::new();

        for rule in &rules {
            assert!(
                codes.insert(&rule.code),
                "Duplicated code rule: '{}'",
                rule.code
            );

            assert!(!rule.code.is_empty(), "A rule has an empty code");
            assert!(
                !rule.title.is_empty(),
                "Rule '{}' does not have a title",
                rule.code
            );
            assert!(
                !rule.description.is_empty(),
                "Rule '{}' does not have a description",
                rule.code
            );
            assert!(
                !rule.recommendation.is_empty(),
                "Rule '{}' does not have recommendation",
                rule.code
            );
        }
    }
}
