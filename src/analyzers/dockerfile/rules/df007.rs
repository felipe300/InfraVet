use crate::core::issue::{Issue, RuleId, Severity};

pub (crate) struct DF007;

impl DF007 {
    pub (crate) fn check(err_msg: &str) -> Issue {
        Issue {
            rule: RuleId::new("DF007"),
            line: 1,
            message: format!("Invalid Dockerfile syntax: {}", err_msg),
            severity: Severity::Error,
        }
    }
}
