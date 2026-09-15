use crate::core::issue::{Issue, RuleId};
use crate::core::rule::DockerfileRule;
use crate::models::Severity;
use crate::utils::utils::uses_latest_tag;
use dockerfile_parser::Instruction;

pub(crate) struct DF003;

impl DockerfileRule for DF003 {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue> {
        if let Instruction::From(from) = instruction {
            let span = from.span;
            let from_str = &content[span.start..span.end];

            if uses_latest_tag(from_str) {
                return Some(Issue {
                    rule: RuleId::new("DF003"),
                    line,
                    message: "Base image uses the 'latest' tag. Consider using a specific version or a slim variant.".into(),
                    severity: Severity::Info,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dockerfile_parser::Dockerfile;

    #[test]
    fn test_df003_triggers_on_latest_tag() {
        let content = "FROM ubuntu:latest";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let instruction = &dockerfile.instructions[0];

        let rule = DF003;
        let issue = rule.check(instruction, content, 1);

        assert!(issue.is_some());
        let issue = issue.unwrap();
        assert_eq!(issue.rule, RuleId::new("DF003"));
        assert_eq!(issue.severity, Severity::Info);
    }

    #[test]
    fn test_df003_ignores_specific_version() {
        let content = "FROM ubuntu:22.04";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let instruction = &dockerfile.instructions[0];

        let rule = DF003;
        let issue = rule.check(instruction, content, 1);

        assert!(issue.is_none());
    }
}
