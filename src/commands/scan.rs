use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::analyzers::dockerfile::parser::analyze_dockerfile;
use crate::models::FileType;
use crate::utils::output;

pub fn scan(file_type: FileType) -> Result<()> {
    let current_dir = env::current_dir()?;

    let target_name = match file_type {
        FileType::Dockerfile => "Dockerfile",

        _ => {
            output::error("This infrastructure type is not implemented yet.");
            return Ok(());
        }
    };

    output::info(&format!(
        "Searching for '{}' recursively in: {}",
        target_name,
        output::highlight_path(&current_dir)
    ));

    let found_files = scan_recursive(&current_dir, target_name);

    if found_files.is_empty() {
        output::error(&format!("No files matching '{}' were found.", target_name));

        return Ok(());
    }

    output::success(&format!("Found {} matching file(s):", found_files.len()));

    for (index, file) in found_files.iter().enumerate() {
        let relative_path = file.strip_prefix(&current_dir).unwrap_or(file);

        println!("\n{}. {}", index + 1, output::highlight_path(relative_path));

        match file_type {
            FileType::Dockerfile => match analyze_dockerfile(file) {
                Ok(issues) => {
                    if issues.is_empty() {
                        output::success("   The Dockerfile has no issues.");
                    } else {
                        for issue in issues {
                            output::issue(&issue.severity, issue.rule, issue.line, &issue.message);
                        }
                    }
                }

                Err(err) => {
                    output::error(&format!("   Error reading file: {}", err));
                }
            },

            _ => {
                output::info("   Analyzer not implemented yet.");
            }
        }
    }

    Ok(())
}

fn scan_recursive(root: &Path, target_name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_hidden_or_ignored(entry));
    // let walker = WalkDir::new(root).into_iter();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_test_structure() -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let root = dir.path().join("test_project");
        fs::create_dir(&root).unwrap();

        // Root files
        File::create(root.join("Dockerfile")).unwrap();
        File::create(root.join("Dockerfile.local")).unwrap();
        File::create(root.join("README.md")).unwrap();
        File::create(root.join(".env")).unwrap();
        File::create(root.join(".gitignore")).unwrap();

        // app/
        let app = root.join("app");
        fs::create_dir(&app).unwrap();
        File::create(app.join("Dockerfile.prod")).unwrap();
        File::create(app.join("main.rs")).unwrap();

        // Ignored directories
        for name in ["target", "node_modules", ".git"] {
            let dir_path = root.join(name);
            fs::create_dir(&dir_path).unwrap();
            File::create(dir_path.join("Dockerfile")).unwrap();
        }

        (dir, root)
    }

    struct DirGuard(PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.0);
        }
    }

    #[test]
    fn test_scan_recursive_finds_dockerfile() {
        let (_dir, root) = create_test_structure();
        let matches = scan_recursive(&root, "Dockerfile");

        assert_eq!(matches.len(), 3);

        assert!(matches.contains(&root.join("Dockerfile")));
        assert!(matches.contains(&root.join("Dockerfile.local")));
        assert!(matches.contains(&root.join("app/Dockerfile.prod")));
    }

    #[test]
    fn test_scan_recursive_ignores_target_and_hidden() {
        let (_dir, root) = create_test_structure();
        let matches = scan_recursive(&root, "Dockerfile");

        // assert!(matches.is_empty());
        assert!(
            !matches
                .iter()
                .any(|path| path.starts_with(root.join("target")))
        );
        assert!(
            !matches
                .iter()
                .any(|path| path.starts_with(root.join("node_modules")))
        );
        assert!(
            !matches
                .iter()
                .any(|path| path.starts_with(root.join(".git")))
        );
    }

    #[test]
    fn test_scan_unsupported_file_type() {
        let result = scan(FileType::Terraform);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scan_execution_in_temp_dir() {
        // This test only checks that `scan` executes without errors.
        // It does not check whether a Dockerfile exists.
        let dir = tempdir().unwrap();

        // Temporarily change the current directory to test `env::current_dir()`.
        let original_dir = env::current_dir().unwrap();
        let _guard = DirGuard(original_dir);

        env::set_current_dir(dir.path()).unwrap();

        let result = scan(FileType::Dockerfile);

        assert!(result.is_ok());
    }
}
