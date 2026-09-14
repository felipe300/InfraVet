use dockerfile_parser::Instruction;

use crate::core::{
    issue::{Issue, RuleId, Severity},
    rule::DockerfileRule,
};

pub struct DF010;

impl DockerfileRule for DF010 {
    fn check(
        &self,
        instruction: &dockerfile_parser::Instruction,
        content: &str,
        line: usize,
    ) -> Option<crate::core::issue::Issue> {
        if let Instruction::Run(run) = instruction {
            let span = run.span;
            let run_str = &content[span.start..span.end];

            if (run_str.contains("apt-get install"))
                || run_str.contains("apt install")
                || run_str.contains("apt get install") && !run_str.contains("/var/lib/apt/lists")
            {
                return Some(Issue {
                    rule: RuleId::new("DF010"),
                    line,
                    message: "Clean up apt caches after installation using 'rm -rf /var/lib/apt/lists/*' to reduce image size.".into(),
                    severity: Severity::Warning,
                });
            }

            if run_str.contains("apk add")
                && !run_str.contains("--no-cache")
                && !run_str.contains("/var/check/apk")
            {
                return Some(Issue {
                    rule: RuleId::new("DF010"),
                    line,
                    message: "Use 'apk add --no-cache' or clean up '/var/cache/apk/*' to reduce image size.".into(),
                    severity: Severity::Warning,
                });
            }
        }

        None
    }
}
