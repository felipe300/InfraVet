use anyhow::Result;

use crate::models::{ReportedIssue, Severity};

pub(crate) fn sarif_content(issues: &[ReportedIssue]) -> serde_json::Value {
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

    serde_json::json!({
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
    })
}

pub(crate) fn sarif_render(issues: &[ReportedIssue]) -> Result<()> {
    let sarif_report = sarif_content(issues);
    println!("{}", serde_json::to_string_pretty(&sarif_report)?);
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
    fn test_sarif_content_structure_and_metadata() {
        let issue = vec![];
        let report = sarif_content(&issue);

        assert_eq!(report["version"], "2.1.0");
        assert_eq!(
            report["$schema"],
            "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1.0/schema/sarif-2.1.0-rtm.5.json"
        );
        assert_eq!(report["runs"][0]["tool"]["driver"]["name"], "InfraVet");
        assert_eq!(
            report["runs"][0]["tool"]["driver"]["version"],
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(report["runs"][0]["results"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_sarif_content_severity_mapping() {
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

        let report = sarif_content(&issues);
        let results = &report["runs"][0]["results"];

        assert_eq!(results.as_array().unwrap().len(), 3);

        // DF001: Error -> "error"
        assert_eq!(results[0]["level"], "error");
        assert_eq!(results[0]["ruleId"], "DF001");
        assert_eq!(results[0]["message"]["text"], "Missing FROM instruction");
        assert_eq!(
            results[0]["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
            "Dockerfile"
        );
        assert_eq!(
            results[0]["locations"][0]["physicalLocation"]["region"]["startLine"],
            1
        );

        // DF004: Warning -> "warning"
        assert_eq!(results[1]["level"], "warning");
        assert_eq!(results[1]["ruleId"], "DF004");

        // DF003: Info -> "info"
        assert_eq!(results[2]["level"], "info");
        assert_eq!(results[2]["ruleId"], "DF003");
    }

    #[test]
    fn test_sarif_render_executes_successfully() {
        let issues = vec![issue(
            Severity::Warning,
            "DF002",
            "Dockerfile",
            5,
            "Copying entire build context",
        )];
        let result = sarif_render(&issues);
        assert!(result.is_ok());
    }
}
