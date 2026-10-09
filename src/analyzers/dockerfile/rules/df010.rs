use dockerfile_parser::Instruction;

use crate::{
    core::{
        issue::{Issue, RuleId},
        rule::DockerfileRule,
    },
    models::Severity,
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
            if let Some(run_str) = content.get(span.start..span.end) {
                if is_apt_istalling(run_str) && !cleans_apt_cache(run_str) {
                    return Some(Issue {
                        rule: RuleId::new("DF010"),
                        line,
                        message: "Clean up apt caches after installation using 'rm -rf /var/lib/apt/lists/*' to reduce image size.".into(),
                        severity: Severity::Warning,
                    });
                }

                if is_apk_installing(run_str) && !cleans_apk_cache(run_str) {
                    return Some(Issue {
                    rule: RuleId::new("DF010"),
                    line,
                    message: "Use 'apk add --no-cache' or clean up '/var/cache/apk/*' to reduce image size.".into(),
                    severity: Severity::Warning,
                });
                }
            }
        }

        None
    }
}

fn is_apt_istalling(cmd: &str) -> bool {
    cmd.contains("apt-get install")
        || cmd.contains("apt install")
        || cmd.contains("apt get install")
}

fn cleans_apt_cache(cmd: &str) -> bool {
    cmd.contains("/var/lib/apt/lists/")
}

fn is_apk_installing(cmd: &str) -> bool {
    cmd.contains("apk add")
}

fn cleans_apk_cache(cmd: &str) -> bool {
    cmd.contains("--no-cache") || cmd.contains("/var/cache/apk")
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
