use crate::core::issue::{Issue, RuleId, Severity};

pub struct DF006;

impl DF006 {
    pub fn check(content: &str) -> Option<Issue> {
        if content.trim().is_empty() {
            Some(Issue {
                rule: RuleId::new("DF006"),
                line: 1,
                message: "Dockerfile is empty.".into(),
                severity: Severity::Error,
            })
        } else {
            None
        }
    }
}
