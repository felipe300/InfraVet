use anyhow::Result;

use crate::models::ReportedIssue;

pub fn json_content(issues: &[ReportedIssue]) -> serde_json::Value {
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

pub fn json_render(issues: &[ReportedIssue]) -> Result<String> {
    let json_report = json_content(issues);
    let output_str = serde_json::to_string_pretty(&json_report)?;
    // println!("{}", serde_json::to_string_pretty(&json_report)?);
    Ok(output_str)
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

        let json = json_content(&issues);

        assert!(json.is_object());
        let results = json["issues"].as_array().unwrap();
        assert_eq!(results.len(), 1);

        assert_eq!(results[0]["severity"], "ERROR");
        assert_eq!(results[0]["rule"], "DF001");
        assert_eq!(results[0]["file_path"], "Dockerfile");
        assert_eq!(results[0]["line"], 1);
        assert_eq!(results[0]["message"], "Missing FROM instruction");
    }

    #[test]
    fn test_json_content_with_empty_issues() {
        let issues = vec![];
        let json = json_content(&issues);

        assert!(json.is_object());
        let results = json["issues"].as_array().unwrap();
        assert_eq!(results.len(), 0);
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

        assert!(json_render(&issues).is_ok());
    }
}
