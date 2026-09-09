use clap::{
    Parser, Subcommand, ValueEnum,
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
    /// Scan infrastructure files recursively.
    ///
    /// Example: `infra_vet scan --type dockerfile`
    Scan {
        /// Type of infrastructure file to scan.
        #[arg(
            short = 't',
            long = "type",
            value_enum,
            default_value_t = FileType::Dockerfile
        )]
        file_type: FileType,
    },
}
