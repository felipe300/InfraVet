use anyhow::Result;

use crate::models::ScanSummary;
use crate::reports::output;

#[allow(clippy::unnecessary_wraps)]
pub fn cli_render(summary: &ScanSummary) -> Result<String> {
    output::render_summary(summary);
    Ok(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ScanSummary;

    fn scan_summary(
        files_scanned: usize,
        total_issues: usize,
        errors: usize,
        warnings: usize,
        info: usize,
    ) -> ScanSummary {
        ScanSummary {
            files_scanned,
            total_issues,
            errors,
            warnings,
            info,
        }
    }

    #[test]
    fn test_cli_render_default_summary() {
        let summary = ScanSummary::default();
        let result = cli_render(&summary);

        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_render_empty() {
        let summary = scan_summary(0, 0, 0, 0, 0);
        let result = cli_render(&summary);

        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_render_success() {
        let summary = scan_summary(5, 10, 3, 1, 7);
        let result = cli_render(&summary);

        assert!(result.is_ok());
    }
}
