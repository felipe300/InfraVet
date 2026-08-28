use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

        // Ejemplo para conectar con el linter/parser en el futuro:
        // match crate::models::DockerfileSummary::parse_from_file(file) {
        //     Ok(summary) => println!("{:#?}", summary),
        //     Err(e) => output::error(&format!("Error procesando {}: {}", file.display(), e)),
        // }
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
