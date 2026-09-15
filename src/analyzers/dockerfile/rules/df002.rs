use crate::core::issue::{Issue, RuleId, Severity};
use crate::core::rule::DockerfileRule;
use dockerfile_parser::Instruction;

pub (crate) struct DF002;

impl DockerfileRule for DF002 {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue> {
        if let Instruction::Copy(copy) = instruction {
            let span = copy.span;
            let copy_str = &content[span.start..span.end];

            if copy_str.contains(". .") {
                return Some(Issue {
                    rule: RuleId::new("DF002"),
                    line,
                    message: "COPY . . copies the entire build context. Consider copying only the required files or directories.".into(),
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
    fn test_df002_triggers_on_copy_all() {
        let content = "COPY . .";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF002.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF002"));
    }
}
