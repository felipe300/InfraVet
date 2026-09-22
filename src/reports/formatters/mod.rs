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

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_issues() -> Vec<ReportedIssue> {
        vec![]
    }

    #[test]
    fn test_render_reports_supports_all_export_formats() {
        let issues = mock_issues();
        let formats = [
            OutputFormat::Csv,
            OutputFormat::Html,
            OutputFormat::Json,
            OutputFormat::Junit,
            OutputFormat::Markdown,
            OutputFormat::Sarif,
        ];

        for format in &formats {
            let result = render_reports(&issues, format);
            assert!(
                result.is_ok(),
                "Error to render report for the format {:?}",
                format
            );
        }
    }

    #[test]
    #[should_panic(expected = "CLI format does not produce a report string")]
    fn test_render_reports_panics_on_cli_format() {
        let issues = mock_issues();

        let _ = render_reports(&issues, &OutputFormat::Cli);
    }
}
