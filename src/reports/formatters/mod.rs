pub(crate) mod csv;
pub(crate) mod html;
pub(crate) mod json;
pub(crate) mod junit;
pub(crate) mod markdown;
pub(crate) mod sarif;

use anyhow::Result;

use crate::models::{OutputFormat, ReportedIssue};

pub(crate) fn render_reports(issues: &[ReportedIssue], format: &OutputFormat) -> Result<()> {
    let content = match format {
        OutputFormat::Json => json::json_render(issues)?,
        OutputFormat::Markdown => markdown::markdown_render(issues)?,
        OutputFormat::Html => html::html_render(issues)?,
        OutputFormat::Csv => csv::csv_render(issues)?,
        OutputFormat::Junit => junit::junit_render(issues)?,
        OutputFormat::Sarif => sarif::sarif_render(issues)?,
        OutputFormat::Cli => unreachable!("CLI format does not produce a report string"),
    };

    Ok(content)
}
