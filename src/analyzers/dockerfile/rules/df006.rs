use crate::{
    core::issue::{Issue, RuleId},
    models::Severity,
};

pub(crate) struct DF006;

impl DF006 {
    pub(crate) fn check(content: &str) -> Option<Issue> {
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
