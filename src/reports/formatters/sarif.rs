use anyhow::Result;

use crate::models::{ReportedIssue, Severity};

pub(crate) fn sarif_render(issues: &[ReportedIssue]) -> Result<()> {
    let results: Vec<serde_json::Value> = issues
        .iter()
        .map(|i| {
            let level = match i.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
            };

            serde_json::json!({
                "ruleId": i.rule,
                "level": level,
                "message": {
                    "text": i.message
                },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {
                            "uri": i.file_path
                        },
                        "region": {
                            "startLine": i.line
                        }
                    }
                }]
            })
        })
        .collect();

    let sarif_report = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1.0/schema/sarif-2.1.0-rtm.5.json",
        "version": "2.1.0",
        "runs": [
            {
                "tool": {
                    "driver": {
                        "name": "InfraVet",
                        "version": env!("CARGO_PKG_VERSION"),
                        "informationUri": "https://github.com/infravet/infravet"
                    }
                },
                "results": results
            }
        ]
    });

    println!("{}", serde_json::to_string_pretty(&sarif_report)?);
    Ok(())
}
