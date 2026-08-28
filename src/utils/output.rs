use colored::Colorize;

pub fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message.bold());
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message.red());
}

pub fn info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}

pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

pub fn highlight_path(path: &str) -> String {
    path.bright_magenta().underline().to_string()
}
