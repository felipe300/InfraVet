use std::fmt;

use crate::models::Severity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuleId(String);

impl RuleId {
    pub(crate) fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) struct Issue {
    pub(crate) rule: RuleId,
    pub(crate) line: usize,
    pub(crate) message: String,
    pub(crate) severity: Severity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_id_creation_and_display() {
        let rule_from_str = RuleId::new("LINT001");
        assert_eq!(rule_from_str.to_string(), "LINT001");

        let rule_from_str = RuleId::new(String::from("LINT002"));
        assert_eq!(rule_from_str.to_string(), "LINT002");
    }

    #[test]
    fn test_rule_id_equality() {
        let rule1 = RuleId::new("LINT001");
        let rule2 = RuleId::new("LINT001");
        let rule3 = RuleId::new("LINT002");

        assert_eq!(rule1, rule2);
        assert_ne!(rule1, rule3);
    }

    #[test]
    fn test_issue_construction() {
        let issue = Issue {
            rule: RuleId::new("LINT003"),
            line: 12,
            message: String::from("Always use a tagged image"),
            severity: Severity::Warning,
        };

        assert_eq!(issue.rule.to_string(), "LINT003");
        assert_eq!(issue.line, 12);
        assert_eq!(issue.message, "Always use a tagged image");
        assert!(matches!(issue.severity, Severity::Warning));
    }
}
