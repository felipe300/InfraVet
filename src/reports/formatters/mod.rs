pub mod cli;
pub mod csv;
pub mod html;
pub mod json;
pub mod junit;
pub mod markdown;
pub mod sarif;

use anyhow::Result;

use crate::models::{OutputFormat, ReportedIssue, ScanSummary};

pub fn render_reports(
    issues: &[ReportedIssue],
    summary: &ScanSummary,
    format: OutputFormat,
) -> Result<String> {
    let content = match format {
        OutputFormat::Json => json::json_render(issues)?,
        OutputFormat::Markdown => markdown::markdown_render(issues)?,
        OutputFormat::Html => html::html_render(issues)?,
        OutputFormat::Csv => csv::csv_render(issues)?,
        OutputFormat::Junit => junit::junit_render(issues)?,
        OutputFormat::Sarif => sarif::sarif_render(issues)?,
        OutputFormat::Cli => cli::cli_render(summary)?,
    };

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_issues() -> Vec<ReportedIssue> {
        vec![]
    }

    #[test]
    fn test_render_reports_supports_all_export_formats() {
        let issues = mock_issues();
        let summary = ScanSummary::new(0, &issues);
        let formats = [
            OutputFormat::Csv,
            OutputFormat::Html,
            OutputFormat::Json,
            OutputFormat::Junit,
            OutputFormat::Markdown,
            OutputFormat::Sarif,
            OutputFormat::Cli,
        ];

        for format in &formats {
            let result = render_reports(&issues, &summary, *format);
            assert!(
                result.is_ok(),
                "Error to render report for the format {format}"
            );
        }
    }
}
