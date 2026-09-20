use anyhow::Result;

use crate::models::{OutputFormat, ReportedIssue};

pub(crate) fn render_reports(issues: &[ReportedIssue], format: &OutputFormat) -> Result<String> {
    let content = match format {
        OutputFormat::Json => json_render(issues)?,
        OutputFormat::Markdown => markdown_render(issues)?,
        OutputFormat::Html => html_render(issues)?,
        OutputFormat::Csv => csv_render(issues)?,
        OutputFormat::Junit => junit_render(issues)?,
        OutputFormat::Sarif => sarif_render(issues)?,
        OutputFormat::Cli => unreachable!("CLI format does not produce a report string"),
    };

    Ok(content)
}
