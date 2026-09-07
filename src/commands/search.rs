use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzer::dockerfile::{Severity, analyze_dockerfile};
use crate::utils::output;

pub fn search(filename: String) -> Result<()> {
    let current_dir = env::current_dir()?;

    output::info(&format!(
        "Buscando '{}' recursivamente en: {}",
        filename,
        output::highlight_path(&current_dir.display().to_string())
    ));

    let found_files = search_recursive(&current_dir, &filename);

    if found_files.is_empty() {
        output::error(&format!(
            "No se encontraron archivos que coincidan con '{}'.",
            filename,
        ));

        return Ok(());
    }

    output::success(&format!("Se encontraron {} archivo(s):", found_files.len()));

    for (index, file) in found_files.iter().enumerate() {
        println!(
            "  {}. {}",
            index + 1,
            output::highlight_path(&file.display().to_string())
        );

        match analyze_dockerfile(file) {
            Ok(issues) => {
                if issues.is_empty() {
                    output::success("     ✓ El Dockerfile no presenta observaciones.");
                } else {
                    for issue in issues {
                        match issue.severity {
                            Severity::Error => {
                                output::error(&format!(
                                    "     [Línea {}] Error: {}",
                                    issue.line, issue.message
                                ));
                            }
                            Severity::Warning => {
                                output::warning(&format!(
                                    "     [Línea {}] Advertencia: {}",
                                    issue.line, issue.message
                                ));
                            }
                        }
                    }
                }
            }

            Err(err) => {
                output::error(&format!("     Error leyendo el archivo: {}", err));
            }
        }
    }

    Ok(())
}

fn search_recursive(root: &Path, target_name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden_or_ignored(e));

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
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
        .map(|s| s.starts_with('.') || s == "target" || s == "node_modules")
        .unwrap_or(false)
}
