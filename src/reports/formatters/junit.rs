use anyhow::Result;
use std::fmt::Write;

use crate::models::{ReportedIssue, Severity};

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn junit_content(issues: &[ReportedIssue]) -> String {
    let errors = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .count();
    let warnings = issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .count();
    let total = issues.len();

    let mut xml = String::new();

    let _ = writeln!(xml, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(
        xml,
        "<testsuites tests=\"{total}\" failures=\"{warnings}\" errors=\"{errors}\">"
    );
    let _ = writeln!(
        xml,
        "  <testsuite name=\"InfraVet Scan\" tests=\"{total}\" failures=\"{warnings}\" errors=\"{errors}\">"
    );

    if issues.is_empty() {
        let _ = writeln!(xml, "    <testcase name=\"InfraVet Execution\" />");
    } else {
        for i in issues {
            let name = format!("{} in {}:{}", i.rule, i.file_path, i.line);
            let _ = writeln!(
                xml,
                "    <testcase name=\"{}\" classname=\"{}\">",
                xml_escape(&name),
                xml_escape(&i.rule)
            );
            match i.severity {
                Severity::Error => {
                    let _ = writeln!(
                        xml,
                        "      <error message=\"{}\" type=\"{}\">{}</error>",
                        xml_escape(&i.message),
                        i.rule,
                        xml_escape(&i.file_path)
                    );
                }
                Severity::Warning => {
                    let _ = writeln!(
                        xml,
                        "      <failure message=\"{}\" type=\"{}\">{}</failure>",
                        xml_escape(&i.message),
                        i.rule,
                        xml_escape(&i.file_path)
                    );
                }
                Severity::Info => {
                    let xml_message = xml_escape(&i.message);
                    let _ = writeln!(xml, "      <system-out>{xml_message}</system-out>");
                }
            }
            let _ = writeln!(xml, "    </testcase>");
        }
    }
    let _ = writeln!(xml, "  </testsuite>\n</testsuites>");

    xml
}

#[allow(clippy::unnecessary_wraps)]
pub fn junit_render(issues: &[ReportedIssue]) -> Result<()> {
    let junit_report = junit_content(issues);
    println!("{junit_report}");
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
    fn test_junit_content_structure_and_metadata() {
        let issues = vec![];
        let report = junit_content(&issues);

        assert!(report.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    }

    #[test]
    fn test_junit_content_severity_mapping() {
        let issues = vec![
            issue(
                Severity::Error,
                "DF001",
                "Dockerfile",
                1,
                "Missing FROM instruction",
            ),
            issue(
                Severity::Warning,
                "DF004",
                "Dockerfile",
                10,
                "Missing USER instruction",
            ),
            issue(
                Severity::Info,
                "DF003",
                "Dockerfile",
                1,
                "Base image uses latest tag",
            ),
        ];

        let report = junit_content(&issues);

        assert!(report.contains("tests=\"3\" failures=\"1\" errors=\"1\""));
        assert!(report.contains("<error message=\"Missing FROM instruction\" type=\"DF001\">"));
        assert!(report.contains("<failure message=\"Missing USER instruction\" type=\"DF004\">"));
        assert!(report.contains("<system-out>Base image uses latest tag</system-out>"));
    }

    #[test]
    fn test_junit_render_executes_successfully() {
        let issues = vec![issue(
            Severity::Warning,
            "DF002",
            "Dockerfile",
            5,
            "Copying entire build context",
        )];
        let result = junit_render(&issues);

        assert!(result.is_ok());
    }
}
