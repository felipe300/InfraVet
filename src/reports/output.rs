use colored::Colorize;
use std::path::Path;

use crate::{
    core::issue::RuleId,
    models::{FileType, RuleDefinition, ScanSummary, Severity},
};

pub fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

#[expect(dead_code)]
pub fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

#[expect(dead_code)]
pub fn issue(severity: &Severity, rule: &RuleId, line: usize, message: &str) {
    let (symbol, label) = match severity {
        Severity::Error => ("✗".red().bold(), "ERROR".red().bold()),
        Severity::Warning => ("⚠".yellow().bold(), "WARNING".yellow().bold()),
        Severity::Info => ("ℹ".cyan().bold(), "INFO".cyan().bold()),
    };

    let rule = rule.to_string().bright_magenta().bold();

    println!("  {symbol} {rule:<6} {label:<8} [line {line}] {message}");
}

#[expect(dead_code)]
pub fn highlight_path(path: &Path) -> String {
    path.display().to_string().cyan().to_string()
}

pub fn rules(rules: &[RuleDefinition]) {
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

        rule_file_type_handler(file_type);

        for rule in file_rules {
            rule_definition(rule);
        }
    }
}

fn rule_file_type_handler(file_type: FileType) {
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

    for line in rule.example.code.lines() {
        println!("    {line}");
    }

    println!();
    println!("{}", "─".repeat(80).dimmed());
    println!();
}

pub fn render_summary(summary: &ScanSummary) -> String {
    let separator = "━".repeat(42);

    let (status, status_color) = if summary.files_found == 0 {
        ("NO FILES FOUND", "info")
    } else if summary.files_failed == summary.files_found {
        ("SCAN FAILED", "error")
    } else if summary.files_failed > 0 {
        ("PARTIAL SCAN", "warning")
    } else {
        ("SCAN COMPLETED", "success")
    };

    let status = match status_color {
        "success" => status.green().bold().to_string(),
        "warning" => status.yellow().bold().to_string(),
        "error" => status.red().bold().to_string(),
        _ => status.cyan().bold().to_string(),
    };

    let title = "INFRAVET · Infrastructure Analysis".green().bold();
    let separator_style = separator.dimmed();
    let section_title = "GENERAL SCAN REPORT".bold();
    let file_analysis_style = "FILE ANALYSIS".bold();
    let files_found_styles = summary.files_found;
    let files_scanned_styles = summary.files_scanned;
    let files_failed_styles = summary.files_failed;
    let total_issued_styles = summary.total_issues;

    format!(
        "\n\
         {title}\n\
         {separator_style}\n\
         {section_title}\n\
         Status: {status}\n\
         \n\
         {file_analysis_style}\n\
         \n\
           Files found:   {files_found_styles}\n\
           Files scanned: {files_scanned_styles}\n\
           Files failed:  {files_failed_styles}\n\
         \n\
           Total issues:  {total_issued_styles}\n\
         \n\
         {separator_style}\n\
           {} {:<12} {}\n\
           {} {:<12} {}\n\
           {} {:<12} {}\n\
         {separator_style}\n",
        "●".red(),
        "Errors",
        summary.errors,
        "●".yellow(),
        "Warnings",
        summary.warnings,
        "●".cyan(),
        "Info",
        summary.info,
    )
}
