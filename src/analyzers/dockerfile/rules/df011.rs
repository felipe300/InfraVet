use crate::analyzers::dockerfile::context::AnalysisContext;
use crate::core::issue::{Issue, RuleId, Severity};

pub (crate) struct DF011;

impl DF011 {
    pub (crate) fn check(ctx: &AnalysisContext) -> Option<Issue> {
        if !ctx.has_healthcheck {
            return Some(Issue {
                rule: RuleId::new("DF011"),
                line: 1,
                message: "No 'HEALTHCHECK' instruction found. Consider adding one to enable container health monitoring.".into(),
                severity: Severity::Info
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triggers_when_missing() {
        let ctx = AnalysisContext::new();
        let issue = DF011::check(&ctx);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF011"));
    }

    #[test]
    fn test_passes_when_present() {
        let mut ctx = AnalysisContext::new();
        ctx.has_healthcheck = true;
        let issue = DF011::check(&ctx);

        assert!(issue.is_none());
    }
}
