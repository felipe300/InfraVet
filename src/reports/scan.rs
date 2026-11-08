use anyhow::Result;
use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzers::dockerfile::parser::analyze_dockerfile;
use crate::core::issue::Issue;
use crate::models::{FileAnalysisResult, FileType, OutputFormat, ReportedIssue, ScanSummary};
use crate::reports::formatters::render_reports;
use crate::reports::output;

pub fn scan(
    targets: &[FileType],
    format: OutputFormat,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let current_dir = env::current_dir()?;
    let found_files = find_files_for_targets(&current_dir, targets);
    let results = analyze_files(&found_files, &current_dir);
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

fn find_files_for_targets(root: &Path, targets: &[FileType]) -> Vec<(FileType, PathBuf)> {
    let mut matches = Vec::new();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_hidden_or_ignored(entry));

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

fn is_hidden_or_ignored(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|name| name.starts_with('.') || name == "target" || name == "node_modules")
}

fn consolidate_results(
    results: Vec<FileAnalysisResult>,
) -> (Vec<ReportedIssue>, ScanSummary, Vec<String>) {
    let files_found = results.len();
    let mut all_issues = Vec::new();
    let mut files_scanned = 0;
    let mut files_failed = 0;
    let mut analysis_errors = Vec::new();

    for result in results {
        match result.result {
            Ok(issues) => {
                files_scanned += 1;
                all_issues.extend(issues);
            }
            Err(error) => {
                files_failed += 1;
                let file = result.file_path;
                analysis_errors.push(format!("Error analysis {file}: {error}"));
            }
        }
    }

    let summary = ScanSummary::new(files_found, files_scanned, files_failed, &all_issues);

    (all_issues, summary, analysis_errors)
}

fn analyze_files(found_files: &[(FileType, PathBuf)], root: &Path) -> Vec<FileAnalysisResult> {
    found_files
        .par_iter()
        .map(|(file_type, path)| {
            let relative_path = path.strip_prefix(root).unwrap_or(path);
            let file_path = relative_path.display().to_string();

            let result = analyze_single_file(*file_type, path)
                .map(|issues| {
                    issues
                        .into_iter()
                        .map(|issue| ReportedIssue {
                            file_path: file_path.clone(),
                            severity: issue.severity,
                            rule: issue.rule.to_string(),
                            line: issue.line,
                            message: issue.message,
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

        let matches = find_files_for_targets(&root, &targets);

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

        assert!(is_hidden_or_ignored(&hidden_entry));
        assert!(is_hidden_or_ignored(&target_entry));
        assert!(!is_hidden_or_ignored(&normal_entry));
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
}
