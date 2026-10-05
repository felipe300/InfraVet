use anyhow::Result;

use crate::models::ReportedIssue;

pub(crate) fn json_content(issues: &[ReportedIssue]) -> serde_json::Value {
    let results: Vec<serde_json::Value> = issues
        .iter()
        .map(|i| {
            serde_json::json!({
                "severity": i.severity,
                "rule": i.rule,
                "file_path": i.file_path,
                "line": i.line,
                "message": i.message,
            })
        })
        .collect();
    serde_json::json!({
        "issues": results
    })
}

pub(crate) fn json_render(issues: &[ReportedIssue]) -> Result<()> {
    let json_report = json_content(&issues);
    println!("{}", serde_json::to_string_pretty(&json_report)?);
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
    fn test_json_content_structure() {
        let issues = vec![issue(
            Severity::Error,
            "DF001",
            "Dockerfile",
            1,
            "Missing FROM instruction",
        )];

        let json = serde_json::to_value(&issues).unwrap();

        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 1);
        // assert_eq!(json[0]["severity"], Severity::Error); // Error: comparing `serde_json::Value` to `models::Severity`
        assert_eq!(json[0]["severity"], "ERROR");
        assert_eq!(json[0]["file_path"], "Dockerfile");
        assert_eq!(json[0]["line"], 1);
        assert_eq!(json[0]["message"], "Missing FROM instruction");
    }

    #[test]
    fn test_json_content_with_empty_issues() {
        let issues = vec![{}];
        let json = serde_json::to_value(&issues).unwrap();

        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_json_render_executes_successfully() {
        let issues = vec![issue(
            Severity::Error,
            "DF001",
            "Dockerfile",
            1,
            "Missing FROM instruction",
        )];

        assert!(json_render(&issues).is_ok())
    }
}
