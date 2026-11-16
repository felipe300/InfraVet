use anyhow::Result;
use clap::Parser;

use crate::{cli::Commands, reports::scan};

mod analyzers;
mod cli;
mod commands;
mod config;
mod core;
mod models;
mod reports;
mod utils;

use cli::Args;
use commands::rules;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Scan {
            path,
            file_type,
            dockerfile,
            compose,
            terraform,
            kubernetes,
            ansible,
            format,
            output,
            config,
        } => {
            let flags = (dockerfile, compose, terraform, kubernetes, ansible);
            let targets = Commands::resolve_targets(file_type, flags);

            scan(&path, &targets, format, output, config.as_ref())?;
        }
        Commands::Rules => rules()?,
    }

    Ok(())
}
