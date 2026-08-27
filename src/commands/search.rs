use anyhow::{Ok, Result};
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn search(filename: String) -> Result<()> {
    let current_dir = env::current_dir()?;

    println!(
        "Buscando '{}' recursivamente en: {}",
        filename,
        current_dir.display()
    );

    let found_files = search_recursive(&current_dir, &filename);

    if found_files.is_empty() {
        println!(
            "No se encontraron archivos que coincidan con '{}'.",
            filename
        );
        return Ok(());
    }

    println!("\nSe encontraron {} archivo(s):", found_files.len());

    for (index, file) in found_files.iter().enumerate() {
        println!(" {}.- {}", index + 1, file.display());

        // Exmaple for linter
        // match DockerfileSummary::parse_from_file(file) {
        //     Ok(summary) => println!("{:#?}", summary),
        //     Err(e) => eprintln!(
        //         "  [!] Error procesando {}: {}",
        //         file.display(),
        //         e
        //     ),
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
