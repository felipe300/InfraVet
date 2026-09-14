use crate::core::issue::{Issue, RuleId, Severity};
use crate::core::rule::DockerfileRule;
use dockerfile_parser::Instruction;

pub struct DF009;

// Keywords to search
const SENSITIVE_KEYWORDS: &[&str] = &[
    "SECRET",
    "PASSWORD",
    "PASSWD",
    "PWD",
    "API_KEY",
    "APIKEY",
    "TOKEN",
    "AUTH",
    "PRIVATE_KEY",
    "CREDENTIAL",
    "ACCESS_KEY",
];

impl DockerfileRule for DF009 {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue> {
        let span = match instruction {
            Instruction::Env(env) => env.span,
            Instruction::Arg(arg) => arg.span,
            _ => return None,
        };

        let command = &content[span.start..span.end];
        let uppercase_command = command.to_uppercase();

        for keyword in SENSITIVE_KEYWORDS {
            if uppercase_command.contains(keyword) {
                return Some(Issue {
                        rule: RuleId::new("DF009"),
                        line,
                        message: "Possible sensitive data or credential found in ENV or Arg instruction. Use secret mounts or runtime environment variables instead.".into(),
                        severity: Severity::Error,
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
    fn test_triggers_on_env_password() {
        let content = "ENV DB_PASSWORD=secret123";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF009.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF009"));
    }

    #[test]
    fn test_triggers_on_env_api_key() {
        let content = "ENV API_KEY=xyz123";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF009.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF009"));
    }

    #[test]
    fn test_passes_on_safe_env() {
        let content = "ENV PORT=8080";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF009.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_none());
    }

    #[test]
    fn test_triggers_on_arg_password() {
        let content = "ARG DB_PASSWORD=secret123";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF009.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF009"));
    }

    #[test]
    fn test_triggers_on_arg_api_key() {
        let content = "ARG API_KEY=xyz123";
        let dockerfile = Dockerfile::parse(content).unwrap();
        let issue = DF009.check(&dockerfile.instructions[0], content, 1);

        assert!(issue.is_some());
        assert_eq!(issue.unwrap().rule, RuleId::new("DF009"));
    }
}
