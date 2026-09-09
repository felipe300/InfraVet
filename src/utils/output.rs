use std::path::Path;

use colored::Colorize;

use crate::analyzer::issue::{RuleId, Severity};

pub fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

pub fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

pub fn issue(severity: &Severity, rule: RuleId, line: usize, message: &str) {
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

pub fn highlight_path(path: &Path) -> String {
    // path.bright_magenta().underline().to_string()
    path.display().to_string().cyan().to_string()
}
