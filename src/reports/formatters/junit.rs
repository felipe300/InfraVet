use anyhow::Result;

use crate::models::{ReportedIssue, Severity};

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[allow(clippy::unnecessary_wraps)]
pub fn junit_render(issues: &[ReportedIssue]) -> Result<()> {
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
    println!("<testsuites tests=\"{total}\" failures=\"{warnings}\" errors=\"{errors}\">");
    println!(
        "  <testsuite name=\"InfraVet Scan\" tests=\"{total}\" failures=\"{warnings}\" errors=\"{errors}\">"
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
                    let xml_message = xml_escape(&i.message);
                    println!("      <system-out>{xml_message}</system-out>");
                }
            }
            println!("    </testcase>");
        }
    }
    println!("  </testsuite>\n</testsuites>");
    Ok(())
}
