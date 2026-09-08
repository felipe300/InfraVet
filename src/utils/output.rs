use colored::Colorize;

use crate::analyzer::issue::Severity;

pub fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

pub fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

pub fn issue(severity: &Severity, rule: &str, line: usize, message: &str) {
    let (symbol, label) = match severity {
        Severity::Error => ("✗".red().bold(), "ERROR".red().bold()),
        Severity::Warning => ("⚠".yellow().bold(), "WARNING".yellow().bold()),
        Severity::Info => ("ℹ".cyan().bold(), "INFO".cyan().bold()),
    };

    let rule = rule.bright_magenta().bold();

    println!(
        "  {} {:<6} {:<8} [line {}] {}",
        symbol, rule, label, line, message
    );
}

pub fn highlight_path(path: &str) -> String {
    path.bright_magenta().underline().to_string()
}
