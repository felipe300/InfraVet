use colored::Colorize;
use std::path::Path;

use crate::{
    core::issue::RuleId,
    models::{FileType, RuleDefinition, Severity},
};

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

pub(crate) fn rules(rules: &[RuleDefinition]) {
    println!();
    println!("{}", "Implemented Rules".green().bold());
    println!();

    for file_type in [
        FileType::Dockerfile,
        FileType::Compose,
        FileType::Terraform,
        FileType::Kubernetes,
        FileType::Ansible,
    ] {
        let file_rules: Vec<&RuleDefinition> = rules
            .iter()
            .filter(|rule| rule.file_type == file_type)
            .collect();

        if file_rules.is_empty() {
            continue;
        }

        rule_file_type_handler(&file_type);

        for rule in file_rules {
            rule_definition(rule);
        }
    }
}

fn rule_file_type_handler(file_type: &FileType) {
    let title = match file_type {
        FileType::Dockerfile => "Dockerfile",
        FileType::Compose => "Compose",
        FileType::Terraform => "Terraform",
        FileType::Kubernetes => "Kubernetes",
        FileType::Ansible => "Ansible",
    };

    println!("{}", title.blue().bold().underline());
    println!("{}", "─".repeat(60).dimmed());
    println!();
}

fn rule_definition(rule: &RuleDefinition) {
    let severity = match rule.severity {
        Severity::Error => "ERROR".red().bold(),
        Severity::Warning => "WARNING".yellow().bold(),
        Severity::Info => "INFO".cyan().bold(),
    };

    println!("  {}  {}", rule.code.bright_magenta().bold(), severity);

    println!("    {} {}", "Title:".blue().bold(), rule.title);
    println!("    {} {}", "Category:".blue().bold(), rule.category);

    println!();
    println!("    {}", "Description:".blue().bold());
    println!("    {}", rule.description);

    println!();
    println!("    {}", "Recommendation:".blue().bold());
    println!("    {}", rule.recommendation);

    println!();
    println!("    {}", "Example:".blue().bold());
    println!("      {}", rule.example.code);

    println!();
    println!("{}", "─".repeat(60).dimmed());
    println!();
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
