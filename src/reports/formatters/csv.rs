use anyhow::Result;
use std::fmt::Write;

use crate::models::ReportedIssue;

/// Garante is a valid csv file
fn escape_csv_field(field: &str) -> String {
    if field.contains('"') || field.contains(',') || field.contains('\n') || field.contains('\r') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn csv_content(issues: &[ReportedIssue]) -> String {
    let mut buffer = String::from("severity,rule,file_path,line,message\n");

    for i in issues {
        let sev = format!("{:?}", i.severity).to_uppercase();

        let file_path = escape_csv_field(&i.file_path);
        let message = escape_csv_field(&i.message);
        let rule = escape_csv_field(&i.rule);

        writeln!(
            buffer,
            "{},{},{},{},{}",
            sev, rule, file_path, i.line, message
        )
        .unwrap();
    }

    buffer
}

pub(crate) fn csv_render(issues: &[ReportedIssue]) -> Result<()> {
    println!("{}", csv_content(issues));
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
    fn test_escape_csv_field_without_special_characters() {
        let field = "src/main.rs";
        assert_eq!(escape_csv_field(field), "src/main.rs");
    }

    #[test]
    fn test_escape_csv_field_with_comma() {
        let field = "src/foo,bar.rs";
        assert_eq!(escape_csv_field(field), "\"src/foo,bar.rs\"");
    }

    #[test]
    fn test_escape_csv_field_with_quotes() {
        let field = "message \"error\"";
        assert_eq!(escape_csv_field(field), "\"message \"\"error\"\"\"");
    }

    #[test]
    fn test_escape_csv_field_with_newline() {
        let field = "line one\rline two";
        assert_eq!(escape_csv_field(field), "\"line one\rline two\"");
    }

    #[test]
    fn test_escape_csv_field_with_carrieage_return() {
        let field = "line one\rline two";
        assert_eq!(escape_csv_field(field), "\"line one\rline two\"");
    }

    #[test]
    fn test_escape_csv_field_with_multiple_special_characters() {
        let field = "error, \"something\"\nnext line";

        assert_eq!(
            escape_csv_field(field),
            "\"error, \"\"something\"\"\nnext line\""
        );
    }

    #[test]
    fn test_csv_content_with_no_issues() {
        let issues = vec![];

        let result = csv_content(&issues);

        assert_eq!(result, "severity,rule,file_path,line,message\n");
    }

    #[test]
    fn test_csv_content_with_one_issue() {
        let issues = vec![issue(
            Severity::Error,
            "no-unused-vars",
            "src/main.rs",
            42,
            "unused variable",
        )];

        let result = csv_content(&issues);

        assert_eq!(
            result,
            "severity,rule,file_path,line,message\n\
             ERROR,no-unused-vars,src/main.rs,42,unused variable\n"
        );
    }

    #[test]
    fn test_csv_content_with_multiple_issues() {
        let issues = vec![
            issue(
                Severity::Error,
                "rule-one",
                "src/main.rs",
                10,
                "first error",
            ),
            issue(
                Severity::Warning,
                "rule-two",
                "src/lib.rs",
                20,
                "second warning",
            ),
        ];

        let result = csv_content(&issues);

        assert_eq!(
            result,
            "severity,rule,file_path,line,message\n\
             ERROR,rule-one,src/main.rs,10,first error\n\
             WARNING,rule-two,src/lib.rs,20,second warning\n"
        );
    }

    #[test]
    fn test_csv_content_escapes_rule() {
        let issues = vec![issue(
            Severity::Error,
            "rule,with,commas",
            "src/main.rs",
            10,
            "message",
        )];

        let result = csv_content(&issues);

        assert_eq!(
            result,
            "severity,rule,file_path,line,message\n\
             ERROR,\"rule,with,commas\",src/main.rs,10,message\n"
        );
    }

    #[test]
    fn test_csv_content_escapes_file_path() {
        let issues = vec![issue(
            Severity::Error,
            "rule",
            "src/foo,bar.rs",
            10,
            "message",
        )];

        let result = csv_content(&issues);

        assert_eq!(
            result,
            "severity,rule,file_path,line,message\n\
             ERROR,rule,\"src/foo,bar.rs\",10,message\n"
        );
    }

    #[test]
    fn test_csv_content_escapes_message() {
        let issues = vec![issue(
            Severity::Error,
            "rule",
            "src/main.rs",
            10,
            "unexpected \"value\", check this\nagain",
        )];

        let result = csv_content(&issues);

        assert_eq!(
            result,
            "severity,rule,file_path,line,message\n\
             ERROR,rule,src/main.rs,10,\"unexpected \"\"value\"\", check this\n\
             again\"\n"
        );
    }
}
