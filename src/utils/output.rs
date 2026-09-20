use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use std::path::Path;

use crate::{
    core::issue::RuleId,
    models::{FileType, OutputFormat, RuleDefinition, Severity},
};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ReportedIssue {
    pub(crate) file_path: String,
    pub(crate) severity: Severity,
    pub(crate) rule: String,
    pub(crate) line: usize,
    pub(crate) message: String,
}

pub(crate) fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub(crate) fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

pub(crate) fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

pub(crate) fn issue(severity: &Severity, rule: &RuleId, line: usize, message: &str) {
    let (symbol, label) = match severity {
        Severity::Error => ("✗".red().bold(), "ERROR".red().bold()),
        Severity::Warning => ("⚠".yellow().bold(), "WARNING".yellow().bold()),
        Severity::Info => ("ℹ".cyan().bold(), "INFO".cyan().bold()),
    };

    let rule = rule.to_string().bright_magenta().bold();

    println!(
        "  {} {:<6} {:<8} [line {}] {}",
        symbol, rule, label, line, message
    );
}

pub(crate) fn print_formatted_issues(
    issues: &[ReportedIssue],
    format: &OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Cli => {}
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(issues)?);
        }
        OutputFormat::Markdown => {
            println!("# InfraVet Analysis Report\n");
            if issues.is_empty() {
                println!("No issues found.");
            } else {
                println!("| Severity | Rule | File | Line | Message |");
                println!("| --- | --- | --- | --- | --- |");
                for i in issues {
                    let sev = format!("{:?}", i.severity).to_uppercase();
                    println!(
                        "| {} | `{}` | `{}` | {} | {} |",
                        sev, i.rule, i.file_path, i.line, i.message
                    );
                }
            }
        }
        OutputFormat::Html => {
            println!(
                "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>InfraVet Report</title>"
            );
            println!(
                "<style>body{{font-family:sans-serif;margin:20px;}} table{{border-collapse:collapse;width:100%;}} th,td{{border:1px solid #ddd;padding:8px;}} th{{background-color:#f2f2f2;}} .ERROR{{color:red;font-weight:bold;}} .WARNING{{color:orange;font-weight:bold;}} .INFO{{color:blue;font-weight:bold;}}</style></head><body>"
            );
            println!("<h1>InfraVet Analysis Report</h1>");
            if issues.is_empty() {
                println!("<p>No issues found.</p>");
            } else {
                println!(
                    "<table><tr><th>Severity</th><th>Rule</th><th>File</th><th>Line</th><th>Message</th></tr>"
                );
                for i in issues {
                    let sev = format!("{:?}", i.severity).to_uppercase();
                    println!(
                        "<tr><td class=\"{}\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                        sev,
                        sev,
                        i.rule,
                        html_escape(&i.file_path),
                        i.line,
                        html_escape(&i.message)
                    );
                }
                println!("</table>");
            }
            println!("</body></html>");
        }
        OutputFormat::Csv => {
            println!("severity,rule,file_path,line,message");
            for i in issues {
                let sev = format!("{:?}", i.severity).to_uppercase();
                println!(
                    "\"{}\",\"{}\",\"{}\",{},\"{}\"",
                    sev,
                    i.rule,
                    i.file_path.replace('"', "\"\""),
                    i.line,
                    i.message.replace('"', "\"\"")
                );
            }
        }
        OutputFormat::Junit => {
            let errors = issues
                .iter()
                .filter(|i| i.severity == Severity::Error)
                .count();
            let warnings = issues
                .iter()
                .filter(|i| i.severity == Severity::Warning)
                .count();
            let total = issues.len();

            println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
            println!(
                "<testsuites tests=\"{}\" failures=\"{}\" errors=\"{}\">",
                total, warnings, errors
            );
            println!(
                "  <testsuite name=\"InfraVet Scan\" tests=\"{}\" failures=\"{}\" errors=\"{}\">",
                total, warnings, errors
            );

            if issues.is_empty() {
                println!("    <testcase name=\"InfraVet Execution\" />");
            } else {
                for i in issues {
                    let name = format!("{} in {}:{}", i.rule, i.file_path, i.line);
                    println!(
                        "    <testcase name=\"{}\" classname=\"{}\">",
                        xml_escape(&name),
                        xml_escape(&i.rule)
                    );
                    match i.severity {
                        Severity::Error => println!(
                            "      <error message=\"{}\" type=\"{}\">{}</error>",
                            xml_escape(&i.message),
                            i.rule,
                            xml_escape(&i.file_path)
                        ),
                        Severity::Warning => println!(
                            "      <failure message=\"{}\" type=\"{}\">{}</failure>",
                            xml_escape(&i.message),
                            i.rule,
                            xml_escape(&i.file_path)
                        ),
                        Severity::Info => {
                            println!("      <system-out>{}</system-out>", xml_escape(&i.message))
                        }
                    }
                    println!("    </testcase>");
                }
            }
            println!("  </testsuite>\n</testsuites>");
        }
        OutputFormat::Sarif => {
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
        }
    }
    Ok(())
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub(crate) fn highlight_path(path: &Path) -> String {
    path.display().to_string().cyan().to_string()
}

pub(crate) fn rules(rules: &[RuleDefinition]) {
    println!();
    println!("{}", "Implemented Rules".green().bold());
    println!();

    for file_type in [
        FileType::Dockerfile,
        FileType::Compose,
        FileType::Terraform,
        FileType::Kubernetes,
        FileType::Ansible,
    ] {
        let file_rules: Vec<&RuleDefinition> = rules
            .iter()
            .filter(|rule| rule.file_type == file_type)
            .collect();

        if file_rules.is_empty() {
            continue;
        }

        rule_file_type_handler(&file_type);

        for rule in file_rules {
            rule_definition(rule);
        }
    }
}

fn rule_file_type_handler(file_type: &FileType) {
    let title = match file_type {
        FileType::Dockerfile => "Dockerfile",
        FileType::Compose => "Compose",
        FileType::Terraform => "Terraform",
        FileType::Kubernetes => "Kubernetes",
        FileType::Ansible => "Ansible",
    };

    println!("{}", title.blue().bold().underline());
    println!("{}", "─".repeat(60).dimmed());
    println!();
}

fn rule_definition(rule: &RuleDefinition) {
    let severity = match rule.severity {
        Severity::Error => "ERROR".red().bold(),
        Severity::Warning => "WARNING".yellow().bold(),
        Severity::Info => "INFO".cyan().bold(),
    };

    println!("  {}  {}", rule.code.bright_magenta().bold(), severity);
    println!("    {} {}", "Title:".blue().bold(), rule.title);
    println!("    {} {}", "Category:".blue().bold(), rule.category);
    println!();
    println!("    {}", "Description:".blue().bold());
    println!("    {}", rule.description);
    println!();
    println!("    {}", "Recommendation:".blue().bold());
    println!("    {}", rule.recommendation);
    println!();
    println!("    {}", "Example:".blue().bold());

    for line in rule.example.code.lines() {
        println!("    {}", line);
    }

    println!();
    println!("{}", "─".repeat(80).dimmed());
    println!();
}
