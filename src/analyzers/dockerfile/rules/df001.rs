use crate::analyzers::dockerfile::context::AnalysisContext;
use crate::core::issue::{Issue, RuleId, Severity};

pub struct DF001;

impl DF001 {
    pub fn check(ctx: &AnalysisContext) -> Option<Issue> {
        if !ctx.has_from {
            Some(Issue {
                rule: RuleId::new("DF001"),
                line: 1,
                message: "Missing FROM instruction. A Dockerfile must define at least one build stage with FROM.".into(),
                severity: Severity::Error,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_df001_missing_from() {
        let ctx = AnalysisContext::new(); // has_from = false por defecto
        let issue = DF001::check(&ctx);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF001"));
    }
}
