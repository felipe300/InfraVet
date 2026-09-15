use colored::Colorize;
use std::path::Path;

use crate::{core::issue::RuleId, models::Severity};

pub(crate) fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub(crate) fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

pub(crate) fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

pub(crate) fn issue(severity: &Severity, rule: RuleId, line: usize, message: &str) {
    let (symbol, label) = match severity {
        Severity::Error => ("✗".red().bold(), "ERROR".red().bold()),
        Severity::Warning => ("⚠".yellow().bold(), "WARNING".yellow().bold()),
        Severity::Info => ("ℹ".cyan().bold(), "INFO".cyan().bold()),
    };

    let rule = rule.to_string().bright_magenta().bold();

    println!(
        "  {} {:<6} {:<8} [line {}] {}",
        symbol, rule, label, line, message
    );
}

pub(crate) fn highlight_path(path: &Path) -> String {
    path.display().to_string().cyan().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_highlight_path() {
        let path = Path::new("/home/user/project/Dockerfile");
        let result = highlight_path(path);

        assert!(result.contains("/home/user/project/Dockerfile"));
    }

    #[test]
    fn test_highlight_path_relative() {
        let path = "app/Dockerfile.dev".to_string();
        let relative_path = Path::new(&path);
        assert!(highlight_path(relative_path).contains(&path));
    }

    #[test]
    fn test_highlight_empty_path() {
        let path = ".".to_string();
        let empty_path = Path::new(&path);
        assert!(highlight_path(empty_path).contains(&path));
    }

    #[test]
    fn test_highlight_path_windows_and_special_chars() {
        let path = "my folder/Docker file".to_string();
        let path_with_spaces = Path::new(&path);
        assert!(highlight_path(path_with_spaces).contains(&path));
    }
}
