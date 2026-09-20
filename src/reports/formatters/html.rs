use anyhow::Result;

use crate::models::ReportedIssue;

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub(crate) fn html_render(issues: &[ReportedIssue]) -> Result<()> {
    {
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
    Ok(())
}
