use anyhow::Result;

use crate::models::ReportedIssue;

pub(crate) fn json_render(issues: &[ReportedIssue]) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(issues)?);
    Ok(())
}
