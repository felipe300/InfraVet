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
