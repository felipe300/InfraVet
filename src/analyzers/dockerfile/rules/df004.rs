use crate::analyzers::dockerfile::context::AnalysisContext;
use crate::core::issue::{Issue, RuleId, Severity};

pub (crate) struct DF004;

impl DF004 {
    pub (crate) fn check(ctx: &AnalysisContext) -> Option<Issue> {
        if ctx.has_from && !ctx.user_in_current_stage {
            let message = if ctx.stages_count > 1 {
                "Missing USER instruction in the final runtime stage. The container will run as root."
            } else {
                "Missing USER instruction. The container will run as root."
            };

            Some(Issue {
                rule: RuleId::new("DF004"),
                line: 1,
                message: message.into(),
                severity: Severity::Warning,
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
    fn test_df004_single_stage_missing_user() {
        let mut ctx = AnalysisContext::new();
        ctx.has_from = true;
        ctx.stages_count = 1;
        ctx.user_in_current_stage = false;

        let issue = DF004::check(&ctx);
        assert!(issue.is_some());
        assert!(issue.unwrap().message.contains("run as root."));
    }
}
