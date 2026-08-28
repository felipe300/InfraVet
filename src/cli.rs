use clap::{
    Parser, Subcommand,
    builder::styling::{AnsiColor, Effects, Styles},
};

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Yellow.on_default())
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = "CLI tool to analyze DevOps Infrastructure",
    styles = cli_styles()
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search for a file recursively.
    ///
    /// Example: `infraVet search -f Dockerfile`
    Search {
        /// Filename to search for.
        #[arg(short = 'f', long, default_value = "Dockerfile")]
        filename: String,
    },
}
