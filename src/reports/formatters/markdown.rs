use anyhow::Result;
use std::fmt::Write;

use crate::models::ReportedIssue;

pub fn markdown_content(issues: &[ReportedIssue]) -> String {
    let mut buffer = String::from("# InfraVet Analysis Report\n");
    if issues.is_empty() {
        let _ = writeln!(buffer, "No issues found.");
    } else {
        let _ = writeln!(buffer, "| Severity | Rule | File | Line | Message |");
        let _ = writeln!(buffer, "| --- | --- | --- | --- | --- |");
        for i in issues {
            let sev = format!("{:?}", i.severity).to_uppercase();
            let _ = writeln!(
                buffer,
                "| {} | `{}` | `{}` | {} | {} |",
                sev, i.rule, i.file_path, i.line, i.message
            );
        }
    }

    buffer
}

#[allow(clippy::unnecessary_wraps)]
pub fn markdown_render(issues: &[ReportedIssue]) -> Result<()> {
    println!("{}", markdown_content(issues));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ReportedIssue, Severity};

    fn issue(
        severity: Severity,
        rule: &str,
        file_path: &str,
        line: usize,
        message: &str,
    ) -> ReportedIssue {
        ReportedIssue {
            severity,
            rule: rule.to_string(),
            file_path: file_path.to_string(),
            line,
            message: message.to_string(),
        }
    }

    #[test]
    fn test_markdown_content_empty() {
        let issues = vec![];
        let result = markdown_content(&issues);

        assert_eq!(result, "# InfraVet Analysis Report\nNo issues found.\n");
    }

    #[test]
    fn test_markdown_content_with_issues() {
        let issues = vec![issue(
            Severity::Error,
            "DF001",
            "Dockerfile",
            1,
            "Missing FROM instruction",
        )];

        let result = markdown_content(&issues);
        let expected = "# InfraVet Analysis Report\n\
            | Severity | Rule | File | Line | Message |\n\
            | --- | --- | --- | --- | --- |\n\
            | ERROR | `DF001` | `Dockerfile` | 1 | Missing FROM instruction |\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_markdown_render_executes_successfully() {
        let issues = vec![issue(
            Severity::Warning,
            "DF002",
            "Dockerfile",
            5,
            "Copying entire build context",
        )];
        let result = markdown_render(&issues);

        assert!(result.is_ok());
    }
}
