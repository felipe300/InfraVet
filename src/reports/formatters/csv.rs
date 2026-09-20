use anyhow::Result;

use crate::models::ReportedIssue;

pub(crate) fn csv_render(issues: &[ReportedIssue]) -> Result<()> {
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
    Ok(())
}
