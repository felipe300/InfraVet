use crate::models::FileType;
use crate::models::OutputFormat;

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
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
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

        /// Output format for results
        #[arg(
            short = 'r',
            long = "format",
            alias = "output",
            value_enum,
            default_value_t = OutputFormat::Cli
        )]
        format: OutputFormat,
    },
    /// List of current rules per DevOps tools
    Rules,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_scan_defaults_to_dockerfile() {
        let args = Args::try_parse_from(["infravet", "scan"]).unwrap();

        match args.command {
            Commands::Scan { file_type, format } => {
                assert_eq!(file_type, FileType::Dockerfile);
            }
            Commands::Rules => {
                todo!()
            }
        }
    }

    #[test]
    fn test_scan_accepts_dockerfile_type() {
        let args = Args::try_parse_from(["infra_vet", "scan", "-t", "dockerfile"]).unwrap();

        match args.command {
            Commands::Scan { file_type, format } => {
                assert_eq!(file_type, FileType::Dockerfile);
            }
            Commands::Rules => {
                todo!()
            }
        };
    }

    // TODO: Enable when Compose analyzer is implemented.
    // Remove #[ignore] when it is implemented.

    #[test]
    #[ignore]
    fn test_scan_accepts_compose_type() {
        let args = Args::try_parse_from(["infra_vet", "scan", "-t", "compose"]).unwrap();

        match args.command {
            Commands::Scan { file_type, format } => {
                assert_eq!(file_type, FileType::Compose);
            }
            Commands::Rules => {
                todo!()
            }
        };
    }

    #[test]
    fn test_scan_accepts_long_argument() {
        let args = Args::try_parse_from(["infra_vet", "scan", "--type", "dockerfile"]).unwrap();

        match args.command {
            Commands::Scan { file_type, format } => {
                assert_eq!(file_type, FileType::Dockerfile);
            }
            Commands::Rules => {
                todo!()
            }
        };
    }

    #[test]
    fn test_invalid_argument() {
        let args = Args::try_parse_from(["", "scan", "-i", "invalid"]);
        assert!(args.is_err());
    }
}
