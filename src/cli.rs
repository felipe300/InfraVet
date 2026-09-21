use crate::models::{FileType, OutputFormat};
use std::path::PathBuf;

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
    name = "infravet",
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
    /// Examples:
    ///   infravet scan                     # Scan all infrastructure types
    ///   infravet scan -d                  # Scan Dockerfiles only
    ///   infravet scan -t terraform        # Scan Terraform files only
    ///   infravet scan -f json -o r.json   # Export JSON report
    Scan {
        /// Target path to scan (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Filter by specific infrastructure type.
        /// If not provided, all supported types will be scanned.
        #[arg(
            short = 't',
            long = "type",
            value_enum,
            conflicts_with_all = ["dockerfile", "compose", "terraform", "kubernetes", "ansible"]
        )]
        file_type: Option<FileType>,

        /// Flag shortcut: Scan Dockerfiles only
        #[arg(short = 'd', long = "dockerfile")]
        dockerfile: bool,

        /// Flag shortcut: Scan Docker Compose files only
        #[arg(short = 'c', long = "compose")]
        compose: bool,

        /// Flag shortcut: Scan Terraform files only (--tf)
        #[arg(short = 'r', long = "terraform", visible_alias = "tf")]
        terraform: bool,

        /// Flag shortcut: Scan Kubernetes files only
        #[arg(short = 'k', long = "kubernetes")]
        kubernetes: bool,

        /// Flag shortcut: Scan Ansible files only
        #[arg(short = 'a', long = "ansible")]
        ansible: bool,

        /// Output format for results (cli, json, html, markdown, csv, junit, sarif)
        #[arg(
            short = 'f',
            long = "format",
            value_enum,
            default_value_t = OutputFormat::Cli
        )]
        format: OutputFormat,

        /// Path to save the report file (e.g. -o report.json)
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// List of current rules per DevOps tools
    Rules,
}

impl Commands {
    pub(crate) fn resolve_targets(
        file_type: Option<FileType>,
        dockerfile: bool,
        compose: bool,
        terraform: bool,
        kubernetes: bool,
        ansible: bool,
    ) -> Vec<FileType> {
        if dockerfile {
            return vec![FileType::Dockerfile];
        }
        if compose {
            return vec![FileType::Compose];
        }
        if terraform {
            return vec![FileType::Terraform];
        }
        if kubernetes {
            return vec![FileType::Kubernetes];
        }
        if ansible {
            return vec![FileType::Ansible];
        }

        if let Some(target) = file_type {
            return vec![target];
        }

        vec![
            FileType::Dockerfile,
            FileType::Compose,
            FileType::Terraform,
            FileType::Kubernetes,
            FileType::Ansible,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_scan_defaults_to_none_file_type() {
        let args = Args::try_parse_from(["infravet", "scan"]).unwrap();

        match args.command {
            Commands::Scan {
                file_type,
                dockerfile,
                ..
            } => {
                assert_eq!(file_type, None);
                assert!(!dockerfile);
            }
            Commands::Rules => panic!("Expected Scan command"),
        }
    }

    #[test]
    fn test_scan_accepts_type_argument() {
        let args = Args::try_parse_from(["infravet", "scan", "-t", "dockerfile"]).unwrap();

        match args.command {
            Commands::Scan { file_type, .. } => {
                assert_eq!(file_type, Some(FileType::Dockerfile));
            }
            Commands::Rules => panic!("Expected Scan command"),
        };
    }

    #[test]
    fn test_scan_accepts_dockerfile_flag() {
        let args = Args::try_parse_from(["infravet", "scan", "-d"]).unwrap();

        match args.command {
            Commands::Scan {
                dockerfile,
                file_type,
                ..
            } => {
                assert!(dockerfile);
                assert_eq!(file_type, None);
            }
            Commands::Rules => panic!("Expected Scan command"),
        };
    }

    #[test]
    fn test_conflicting_flags_fail() {
        let result = Args::try_parse_from(["infravet", "scan", "-t", "dockerfile", "-d"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_accepts_output_file() {
        let args =
            Args::try_parse_from(["infravet", "scan", "-f", "json", "-o", "report.json"]).unwrap();

        match args.command {
            Commands::Scan { format, output, .. } => {
                assert_eq!(format, OutputFormat::Json);
                assert_eq!(output, Some(PathBuf::from("report.json")));
            }
            Commands::Rules => panic!("Expected Scan command"),
        };
    }

    #[test]
    fn test_invalid_argument() {
        let args = Args::try_parse_from(["infravet", "scan", "-i", "invalid"]);
        assert!(args.is_err());
    }
}
