use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cli => write!(f, "cli"),
            Self::Json => write!(f, "json"),
            Self::Markdown => write!(f, "markdown"),
            Self::Html => write!(f, "html"),
            Self::Csv => write!(f, "csv"),
            Self::Junit => write!(f, "junit"),
            Self::Sarif => write!(f, "sarif"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    #[value(name = "cli")]
    Cli,
    #[value(name = "json")]
    Json,
    #[value(name = "markdown")]
    Markdown,
    #[value(name = "html")]
    Html,
    #[value(name = "csv")]
    Csv,
    #[value(name = "junit")]
    Junit,
    #[value(name = "sarif")]
    Sarif,
}

#[derive(Debug, Clone, Copy, ValueEnum, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    #[value(name = "dockerfile")]
    Dockerfile,

    #[value(name = "compose")]
    Compose,

    #[value(name = "terraform")]
    Terraform,

    #[value(name = "kubernetes")]
    Kubernetes,

    #[value(name = "ansible")]
    Ansible,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Deserialize, Debug)]
pub struct RulesConfig {
    pub(crate) rules: Vec<RuleDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct RuleDefinition {
    pub(crate) code: String,
    pub(crate) file_type: FileType,
    pub(crate) category: String,
    pub(crate) severity: Severity,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) recommendation: String,
    pub(crate) example: Example,
}

#[derive(Deserialize, Debug)]
pub struct Example {
    pub(crate) code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportedIssue {
    pub(crate) file_path: String,
    pub(crate) severity: Severity,
    pub(crate) rule: String,
    pub(crate) line: usize,
    pub(crate) message: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ScanSummary {
    pub files_found: usize,
    pub files_scanned: usize,
    pub files_failed: usize,

    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
}

impl ScanSummary {
    pub fn new(
        files_found: usize,
        files_scanned: usize,
        files_failed: usize,
        issues: &[ReportedIssue],
    ) -> Self {
        let mut summary = Self {
            files_found,
            files_scanned,
            files_failed,
            total_issues: issues.len(),
            ..Self::default()
        };

        for issue in issues {
            match issue.severity {
                // Severity::Error => summary.errors = summary.errors.saturating_add(1),
                // Severity::Warning => summary.warnings = summary.warnings.saturating_add(1),
                // Severity::Info => summary.info = summary.info.saturating_add(1),
                Severity::Error => summary.errors += 1,
                Severity::Warning => summary.warnings += 1,
                Severity::Info => summary.info += 1,
            }
        }

        summary
    }
}

#[derive(Debug)]
pub struct FileAnalysisResult {
    pub file_path: String,
    pub result: std::result::Result<Vec<ReportedIssue>, String>,
}
