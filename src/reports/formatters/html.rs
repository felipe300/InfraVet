use anyhow::Result;
use std::fmt::Write;

use crate::models::ReportedIssue;

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn html_content(issues: &[ReportedIssue]) -> String {
    let mut buffer = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>InfraVet Report</title>",
    );

    let _ = writeln!(
        buffer,
        "<style>body{{font-family:sans-serif;margin:20px;}} table{{border-collapse:collapse;width:100%;}} th,td{{border:1px solid #ddd;padding:8px;}} th{{background-color:#f2f2f2;}} .ERROR{{color:red;font-weight:bold;}} .WARNING{{color:orange;font-weight:bold;}} .INFO{{color:blue;font-weight:bold;}}</style></head><body>"
    );
    let _ = writeln!(buffer, "<h1>InfraVet Analysis Report</h1>");

    if issues.is_empty() {
        let _ = writeln!(buffer, "<p>No issues found.</p>");
    } else {
        let _ = writeln!(
            buffer,
            "<table><tr><th>Severity</th><th>Rule</th><th>File</th><th>Line</th><th>Message</th></tr>"
        );
        for i in issues {
            let sev = format!("{:?}", i.severity).to_uppercase();
            let _ = writeln!(
                buffer,
                "<tr><td class=\"{}\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                sev,
                sev,
                html_escape(&i.rule),
                html_escape(&i.file_path),
                i.line,
                html_escape(&i.message)
            );
        }
        let _ = writeln!(buffer, "</table>");
    }
    let _ = writeln!(buffer, "</body></html>");

    buffer
}

#[allow(clippy::unnecessary_wraps)]
pub fn html_render(issues: &[ReportedIssue]) -> Result<()> {
    println!("{}", html_content(issues));
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
    fn test_html_content_empty() {
        let issues = vec![];
        let result = html_content(&issues);
        assert!(result.contains("<p>No issues found.</p>"));
        assert!(result.contains("InfraVet Report"));
    }

    #[test]
    fn test_html_content_with_issues() {
        let issues = vec![issue(
            Severity::Error,
            "DF001",
            "Dockerfile",
            1,
            "Missing FROM instruction",
        )];

        let result = html_content(&issues);
        assert!(result.contains("DF001"));
        assert!(result.contains("Missing FROM instruction"));
        assert!(result.contains("<td class=\"ERROR\">ERROR</td>"));
        assert!(result.contains("Dockerfile"));
    }

    #[test]
    fn test_html_render_executes_successfully() {
        let issues = vec![issue(
            Severity::Warning,
            "DF002",
            "Dockerfile",
            5,
            "Copying entire build context",
        )];
        let result = html_render(&issues);
        assert!(result.is_ok());
    }
}
