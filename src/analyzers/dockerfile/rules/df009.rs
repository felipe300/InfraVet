use crate::core::issue::{Issue, RuleId, Severity};
use crate::core::rule::DockerfileRule;
use dockerfile_parser::Instruction;

pub struct DF009;

impl DockerfileRule for DF009 {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue> {
        if let Instruction::Misc(misc) = instruction {
            let command_name = misc.instruction.content.as_str();

            if !command_name.eq_ignore_ascii_case("ADD") {
                return None;
            }

            let span = misc.span;
            let add_str = &content[span.start..span.end];
            let mut parts = add_str.split_whitespace();

            parts.next();

            let source = parts.next()?;

            let is_remote_or = source.contains("http://") || source.contains("https://");

            let is_archive = source.ends_with(".tar")
                || source.ends_with(".tar.gz")
                || source.ends_with(".tgz")
                || source.ends_with(".zip");

            if !is_remote_or && !is_archive {
                return Some(Issue {
                        rule: RuleId::new("DF008"),
                        line,
                        message: "Use 'COPY' instead of 'ADD' for local files and directories unless extracting archives or fetching remote URLs.".into(),
                        severity: Severity::Warning,
                    });
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dockerfile_parser::Dockerfile;

    #[test]
    fn test_triggers_on_simple_add() {
        let content = "ADD app.js /app/";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF008.check(&dockerfile.instructions[0], content, 0);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF008"));
    }

    #[test]
    fn test_allows_tar_gz_archive() {
        let content = "ADD archive.tar.gz /app/";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF008.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_none());
    }

    #[test]
    fn test_allows_url() {
        let content = "ADD https://example.com/file.txt /app/";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF008.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_none());
    }
}
