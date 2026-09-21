use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzers::dockerfile::parser::analyze_dockerfile;
use crate::core::issue::Issue;
use crate::models::{FileType, OutputFormat, ReportedIssue};
use crate::reports::formatters::render_reports;
use crate::reports::output;

pub(crate) fn scan(targets: &[FileType], format: OutputFormat) -> Result<()> {
    let current_dir = env::current_dir()?;
    let is_cli = format == OutputFormat::Cli;

    if is_cli {
        output::info(&format!(
            "Searching for infrastructure files in: {}",
            output::highlight_path(&current_dir)
        ));
    }

    let found_files = find_files_for_targets(&current_dir, targets);

    if found_files.is_empty() {
        if is_cli {
            output::error("No infrastructure files matching the criteria were found.");
        } else {
            render_reports(&[], &format)?;
        }
        return Ok(());
    }

    let mut all_issues: Vec<ReportedIssue> = Vec::new();

    for (file_type, path) in &found_files {
        let relative_path = path.strip_prefix(&current_dir).unwrap_or(path);
        let path_str = relative_path.display().to_string();

        if is_cli {
            println!("\n {}", output::highlight_path(relative_path));
        }

        match analyze_single_file(file_type, path) {
            Ok(issues) => {
                if issues.is_empty() {
                    if is_cli {
                        output::success("    No issues found.");
                    }
                } else {
                    for issue in issues {
                        if is_cli {
                            output::issue(&issue.severity, &issue.rule, issue.line, &issue.message);
                        }
                        all_issues.push(ReportedIssue {
                            file_path: path_str.clone(),
                            severity: issue.severity.clone(),
                            rule: issue.rule.to_string(),
                            line: issue.line,
                            message: issue.message.clone(),
                        });
                    }
                }
            }
            Err(err) => {
                if is_cli {
                    output::error(&format!("    Error analyzing file: {}", err));
                }
            }
        }
    }

    if !is_cli {
        render_reports(&all_issues, &format)?;
    }

    Ok(())
}

fn analyze_single_file(file_type: &FileType, path: &Path) -> Result<Vec<Issue>> {
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

    for entry in walker.filter_map(|entry| entry.ok()) {
        let path = entry.path();

        if path.is_file() {
            for target in targets {
                if match_file_type(path, target) {
                    matches.push((target.clone(), path.to_path_buf()));
                    break;
                }
            }
        }
    }

    matches
}

fn match_file_type(path: &Path, file_type: &FileType) -> bool {
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return false,
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
        .map(|name| name.starts_with('.') || name == "target" || name == "node_modules")
        .unwrap_or(false)
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
}
