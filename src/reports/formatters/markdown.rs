use anyhow::Result;

use crate::models::ReportedIssue;

pub(crate) fn markdown_render(issues: &[ReportedIssue]) -> Result<()> {
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
    Ok(())
}
