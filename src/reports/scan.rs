use anyhow::Result;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzers::dockerfile::parser::analyze_dockerfile;
use crate::config::loader::{FileConfig, RuleConfigurationOverride};
use crate::core::issue::Issue;
use crate::models::{FileAnalysisResult, FileType, OutputFormat, ReportedIssue, ScanSummary};
use crate::reports::formatters::render_reports;
use crate::reports::output;

pub fn scan(
    target_path: &Path,
    targets: &[FileType],
    format: OutputFormat,
    output_path: Option<PathBuf>,
    config_path: Option<&PathBuf>,
) -> Result<()> {
    let root = target_path.canonicalize()?;

    let config = FileConfig::load_for_scan(&root, config_path.map(std::convert::AsRef::as_ref))?;

    let found_files = find_files_for_targets(&root, targets, &config.ignore.paths);
    let results = analyze_files(&found_files, &root, &config);
    let (all_issues, summary, analysis_errors) = consolidate_results(results);
    let report_content = render_reports(&all_issues, &summary, format)?;

    for error in analysis_errors {
        output::error(&error);
    }

    if let Some(path) = output_path {
        std::fs::write(&path, &report_content)?;
        output::success(&format!(
            "Report successfully written to {}",
            path.display()
        ));
    } else {
        print!("{report_content}");
    }

    Ok(())
}

fn analyze_single_file(file_type: FileType, path: &Path) -> Result<Vec<Issue>> {
    match file_type {
        FileType::Dockerfile => analyze_dockerfile(path),
        FileType::Compose => {
            // TODO: Invocación a analyze_compose(path) cuando esté listo
            Ok(Vec::new())
        }
        FileType::Terraform => {
            // TODO: Invocación a analyze_terraform(path) cuando esté listo
            Ok(Vec::new())
        }
        FileType::Kubernetes => {
            // TODO: Invocación a analyze_kubernetes(path) cuando esté listo
            Ok(Vec::new())
        }
        FileType::Ansible => {
            // TODO: Invocación a analyze_ansible(path) cuando esté listo
            Ok(Vec::new())
        }
    }
}

fn find_files_for_targets(
    root: &Path,
    targets: &[FileType],
    ignored_paths: &[String],
) -> Vec<(FileType, PathBuf)> {
    let mut matches = Vec::new();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_hidden_or_ignored(entry, ignored_paths, root));

    for entry in walker.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            for target in targets {
                if match_file_type(path, *target) {
                    matches.push((*target, path.to_path_buf()));
                    break;
                }
            }
        }
    }

    matches
}

fn match_file_type(path: &Path, file_type: FileType) -> bool {
    let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };

    match file_type {
        FileType::Dockerfile => file_name == "Dockerfile" || file_name.starts_with("Dockerfile."),
        FileType::Compose => {
            file_name == "docker-compose.yml"
                || file_name == "docker-compose.yaml"
                || file_name.starts_with("compose.")
        }
        FileType::Terraform => path.extension().and_then(|e| e.to_str()) == Some("tf"),
        FileType::Kubernetes => {
            path.extension().and_then(|e| e.to_str()) == Some("yaml")
                || path.extension().and_then(|e| e.to_str()) == Some("yml")
        }
        FileType::Ansible => {
            file_name.contains("playbook") || file_name == "site.yml" || file_name == "site.yaml"
        }
    }
}

fn is_hidden_or_ignored(entry: &walkdir::DirEntry, ignored_paths: &[String], root: &Path) -> bool {
    let is_default_ignored = entry
        .file_name()
        .to_str()
        .is_some_and(|name| name.starts_with('.') || name == "target" || name == "node_modules");

    if is_default_ignored {
        return true;
    }

    let path = entry.path();
    let relative_path = path.strip_prefix(root).unwrap_or(path);
    let path_str = relative_path.to_string_lossy();

    ignored_paths.iter().any(|ignored| {
        let cleaned_ignored = ignored.trim_end_matches('/');
        path_str == cleaned_ignored || path_str.starts_with(&format!("{cleaned_ignored}/"))
    })
}

fn consolidate_results(
    results: Vec<FileAnalysisResult>,
) -> (Vec<ReportedIssue>, ScanSummary, Vec<String>) {
    let files_found = results.len();
    let mut all_issues = Vec::new();
    let mut files_scanned: usize = 0;
    let mut files_failed: usize = 0;
    let mut analysis_errors = Vec::new();

    for result in results {
        match result.result {
            Ok(issues) => {
                files_scanned = files_scanned.saturating_add(1);
                all_issues.extend(issues);
            }
            Err(error) => {
                files_failed = files_failed.saturating_add(1);
                let file = result.file_path;
                analysis_errors.push(format!("Error analysis {file}: {error}"));
            }
        }
    }

    let summary = ScanSummary::new(files_found, files_scanned, files_failed, &all_issues);

    (all_issues, summary, analysis_errors)
}

fn analyze_files(
    found_files: &[(FileType, PathBuf)],
    root: &Path,
    config: &FileConfig,
) -> Vec<FileAnalysisResult> {
    found_files
        .par_iter()
        .map(|(file_type, path)| {
            let relative_path = path.strip_prefix(root).unwrap_or(path);
            let file_path = relative_path.display().to_string();

            let result = analyze_single_file(*file_type, path)
                .map(|issues| {
                    issues
                        .into_iter()
                        .filter_map(|issue| {
                            let rule_key = &issue.rule;
                            if let Some(rule_override) = config.rules.get(rule_key) {
                                match rule_override {
                                    RuleConfigurationOverride::Severity(custom_severity) => {
                                        Some(ReportedIssue {
                                            file_path: file_path.clone(),
                                            severity: custom_severity.clone(),
                                            rule: issue.rule.to_string(),
                                            line: issue.line,
                                            message: issue.message,
                                        })
                                    }
                                    RuleConfigurationOverride::Detailed { enabled, severity } => {
                                        if !*enabled {
                                            return None;
                                        }
                                        let final_severity =
                                            severity.clone().unwrap_or(issue.severity);
                                        Some(ReportedIssue {
                                            file_path: file_path.clone(),
                                            severity: final_severity,
                                            rule: issue.rule.to_string(),
                                            line: issue.line,
                                            message: issue.message,
                                        })
                                    }
                                }
                            } else {
                                Some(ReportedIssue {
                                    file_path: file_path.clone(),
                                    severity: issue.severity,
                                    rule: issue.rule.to_string(),
                                    line: issue.line,
                                    message: issue.message,
                                })
                            }
                        })
                        .collect()
                })
                .map_err(|error| error.to_string());

            FileAnalysisResult { file_path, result }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::core::issue;

    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_test_structure() -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let root = dir.path().join("test_project");
        fs::create_dir(&root).unwrap();

        File::create(root.join("Dockerfile")).unwrap();
        File::create(root.join("Dockerfile.local")).unwrap();
        File::create(root.join("docker-compose.yml")).unwrap();
        File::create(root.join("main.tf")).unwrap();
        File::create(root.join("README.md")).unwrap();

        let app = root.join("app");
        fs::create_dir(&app).unwrap();
        File::create(app.join("Dockerfile.prod")).unwrap();

        for name in ["target", "node_modules", ".git"] {
            let dir_path = root.join(name);
            fs::create_dir(&dir_path).unwrap();
            File::create(dir_path.join("Dockerfile")).unwrap();
        }

        (dir, root)
    }

    #[test]
    fn test_find_files_for_targets() {
        let (_dir, root) = create_test_structure();
        let targets = vec![FileType::Dockerfile, FileType::Compose];
        let ignored_paths: &[String] = &[];

        let matches = find_files_for_targets(&root, &targets, ignored_paths);

        assert_eq!(matches.len(), 4);
    }

    #[test]
    fn test_match_file_type_variations() {
        assert!(match_file_type(
            Path::new("Dockerfile"),
            FileType::Dockerfile
        ));
        assert!(match_file_type(
            Path::new("Dockerfile.dev"),
            FileType::Dockerfile
        ));
        assert!(match_file_type(
            Path::new("docker-compose.yml"),
            FileType::Compose
        ));
        assert!(match_file_type(
            Path::new("compose.override.yaml"),
            FileType::Compose
        ));
        assert!(match_file_type(Path::new("main.tf"), FileType::Terraform));
        assert!(match_file_type(
            Path::new("deployment.yml"),
            FileType::Kubernetes
        ));
        assert!(match_file_type(Path::new("site.yaml"), FileType::Ansible));
        assert!(match_file_type(
            Path::new("web_playbook.yml"),
            FileType::Ansible
        ));

        assert!(!match_file_type(Path::new("main.rs"), FileType::Dockerfile));
        assert!(!match_file_type(
            Path::new("values.json"),
            FileType::Kubernetes
        ));
    }

    #[test]
    fn test_is_hidden_or_ignored() {
        let (_dir, root) = create_test_structure();

        let hidden_entry = WalkDir::new(root.join(".git"))
            .into_iter()
            .next()
            .unwrap()
            .unwrap();
        let target_entry = WalkDir::new(root.join("target"))
            .into_iter()
            .next()
            .unwrap()
            .unwrap();
        let normal_entry = WalkDir::new(root.join("Dockerfile"))
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        let ignored_paths: &[String] = &[];

        assert!(is_hidden_or_ignored(&hidden_entry, ignored_paths, &root));
        assert!(is_hidden_or_ignored(&target_entry, ignored_paths, &root));
        assert!(!is_hidden_or_ignored(&normal_entry, ignored_paths, &root));
    }

    #[test]
    fn test_consolidate_results_with_failed_file() {
        let results = vec![
            FileAnalysisResult {
                file_path: "Dockerfile".to_string(),
                result: Ok(vec![]),
            },
            FileAnalysisResult {
                file_path: "Dockerfile.prod".to_string(),
                result: Err("Permission denied".to_string()),
            },
        ];

        let (issues, summary, errors) = consolidate_results(results);

        assert!(issues.is_empty());
        assert_eq!(summary.files_found, 2);
        assert_eq!(summary.files_scanned, 1);
        assert_eq!(summary.files_failed, 1);
        assert_eq!(summary.total_issues, 0);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_scan_respects_infravet_toml_ignore_paths() {
        let dir = tempdir().unwrap();

        let root = dir.path().join("test_project");
        fs::create_dir(&root).unwrap();
        File::create(root.join("Dockerfile")).unwrap();

        let exlcude_dir = root.join("legacy_infra");
        fs::create_dir(&exlcude_dir).unwrap();
        File::create(root.join("Dockerfile")).unwrap();

        let config_content = r#"
            [ignore]
            paths = ["legacy_infra"]
        "#;
        fs::write(root.join("infravet"), config_content).unwrap();

        let config = FileConfig::load_from_file(root.join("infravet.toml")).unwrap_or_default();
        let targets = vec![FileType::Dockerfile];
        let found_files = find_files_for_targets(&root, &targets, &config.ignore.paths);

        assert_eq!(found_files.len(), 1);
        assert!(found_files[0].1.ends_with("Dockerfile"));
        assert!(!found_files[0].1.to_string_lossy().contains("legacy_infra"));
    }

    #[test]
    fn test_scan_applies_infravet_toml_rules_and_overrides() {
        use crate::config::loader::FileConfig;
        use crate::core::issue::Issue;
        use crate::models::Severity;

        let dir = tempdir().unwrap();
        let root = dir.path().join("test_project");
        fs::create_dir(&root).unwrap();

        let config_content = r#"
            default-severity = "WARNING"

            [rules]
            "DF002" = { enabled = false }
            "DF003" = "ERROR"
        "#;
        fs::write(root.join("infravet.toml"), config_content).unwrap();

        let config = FileConfig::load_for_scan(&root, None).unwrap();

        let simulated_issues = vec![
            Issue {
                rule: issue::RuleId("DF001".to_string()),
                severity: Severity::Error,
                line: 1,
                message: "Missing FROM".to_string(),
            },
            Issue {
                rule: issue::RuleId("DF002".to_string()),
                severity: Severity::Warning,
                line: 5,
                message: "Copy entire context".to_string(),
            },
            Issue {
                rule: issue::RuleId("DF003".to_string()),
                severity: Severity::Info,
                line: 10,
                message: "Latest tag used".to_string(),
            },
        ];

        let filtered_issues: Vec<ReportedIssue> = simulated_issues
            .into_iter()
            .filter_map(|issue| {
                let rule_code = issue.rule.to_string();
                if let Some(rule_override) = config.rules.get(&issue::RuleId(rule_code)) {
                    match rule_override {
                        RuleConfigurationOverride::Severity(custom_severity) => {
                            Some(ReportedIssue {
                                file_path: "Dockerfile".to_string(),
                                severity: custom_severity.clone(),
                                rule: issue.rule.to_string(),
                                line: issue.line,
                                message: issue.message,
                            })
                        }
                        RuleConfigurationOverride::Detailed { enabled, severity } => {
                            if !*enabled {
                                return None;
                            }
                            let final_severity = severity.clone().unwrap_or(issue.severity);
                            Some(ReportedIssue {
                                file_path: "Dockerfile".to_string(),
                                severity: final_severity,
                                rule: issue.rule.to_string(),
                                line: issue.line,
                                message: issue.message,
                            })
                        }
                    }
                } else {
                    Some(ReportedIssue {
                        file_path: "Dockerfile".to_string(),
                        severity: issue.severity,
                        rule: issue.rule.to_string(),
                        line: issue.line,
                        message: issue.message,
                    })
                }
            })
            .collect();

        assert_eq!(filtered_issues.len(), 2);

        assert!(!filtered_issues.iter().any(|i| i.rule == "DF002"));

        let df003_issue = filtered_issues.iter().find(|i| i.rule == "DF003").unwrap();
        assert_eq!(df003_issue.severity, Severity::Error);
    }
}
