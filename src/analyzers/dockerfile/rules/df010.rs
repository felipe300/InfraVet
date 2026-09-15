use dockerfile_parser::Instruction;

use crate::{
    core::{
        issue::{Issue, RuleId},
        rule::DockerfileRule,
    },
    models::Severity,
};

pub(crate) struct DF010;

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

            let has_apt_install = run_str.contains("apt-get install")
                || run_str.contains("apt install")
                || run_str.contains("apt get install");

            let has_apt_cleanup = run_str.contains("/var/lib/apt/lists");

            if has_apt_install && !has_apt_cleanup {
                return Some(Issue {
                    rule: RuleId::new("DF010"),
                    line,
                    message: "Clean up apt caches after installation using 'rm -rf /var/lib/apt/lists/*' to reduce image size.".into(),
                    severity: Severity::Warning,
                });
            }

            let has_apk_install = run_str.contains("apk add");
            let has_apk_no_cache = run_str.contains("--no-cache");
            let has_apk_cleanup = run_str.contains("/var/cache/apk");

            if has_apk_install && !has_apk_no_cache && !has_apk_cleanup {
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

#[cfg(test)]
mod tests {
    use super::*;
    use dockerfile_parser::Dockerfile;

    #[test]
    fn test_triggers_on_uncleaned_opt() {
        let content = "RUN apt-get update && apt-get install -y curl";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF010.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF010"));
    }

    #[test]
    fn test_passes_on_clean_apt() {
        let content =
            "RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF010.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_none());
    }

    #[test]
    fn test_passes_on_apk_no_cache() {
        let content = "RUN apk add --no-cache curl";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF010.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_none());
    }
}
