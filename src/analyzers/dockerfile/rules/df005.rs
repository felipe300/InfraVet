use crate::core::issue::{Issue, RuleId};
use crate::core::rule::DockerfileRule;
use crate::models::Severity;
use dockerfile_parser::Instruction;

pub(crate) struct DF005;

impl DockerfileRule for DF005 {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue> {
        if let Instruction::Run(run) = instruction {
            let span = run.span;
            let command = &content[span.start..span.end];

            if command.contains("apt-get update") && !command.contains("apt-get install") {
                return Some(Issue {
                    rule: RuleId::new("DF005"),
                    line,
                    message: "'apt-get update' is used without 'apt-get install' in the same RUN instruction.".into(),
                    severity: Severity::Warning,
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
    fn test_df005_triggers_on_unpaired_apt_update() {
        let content = "RUN apt-get update";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF005.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF005"));
    }
}
