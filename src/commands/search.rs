use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzer::dockerfile::analyze_dockerfile;
use crate::utils::output;

pub fn search(filename: String) -> Result<()> {
    let current_dir = env::current_dir()?;

    output::info(&format!(
        "Searching for '{}' recursively in: {}",
        filename,
        output::highlight_path(&current_dir.display().to_string())
    ));

    let found_files = search_recursive(&current_dir, &filename);

    if found_files.is_empty() {
        output::error(&format!("No files matching '{}' were found.", filename));

        return Ok(());
    }

    output::success(&format!("Found {} matching file(s):", found_files.len()));

    for (index, file) in found_files.iter().enumerate() {
        println!(
            "\n{}. {}",
            index + 1,
            output::highlight_path(&file.display().to_string())
        );

        match analyze_dockerfile(file) {
            Ok(issues) => {
                if issues.is_empty() {
                    output::success("   The Dockerfile has no issues.");
                } else {
                    for issue in issues {
                        output::issue(
                            &issue.severity,
                            &issue.rule.to_string(),
                            issue.line,
                            &issue.message,
                        );
                    }
                }
            }

            Err(err) => {
                output::error(&format!("   Error reading file: {}", err));
            }
        }
    }

    Ok(())
}

fn search_recursive(root: &Path, target_name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_hidden_or_ignored(entry));

    for entry in walker.filter_map(|entry| entry.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                if name == target_name || name.starts_with(&format!("{}.", target_name)) {
                    matches.push(path.to_path_buf());
                }
            }
        }
    }

    matches
}

fn is_hidden_or_ignored(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.') || name == "target" || name == "node_modules")
        .unwrap_or(false)
}
