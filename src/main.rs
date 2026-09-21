use anyhow::Result;
use clap::Parser;

use crate::{cli::Commands, reports::scan};

mod analyzers;
mod cli;
mod commands;
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
            file_type,
            dockerfile,
            compose,
            terraform,
            kubernetes,
            ansible,
            format,
            output: _,
            path: _,
        } => {
            let targets = Commands::resolve_targets(
                file_type, dockerfile, compose, terraform, kubernetes, ansible,
            );

            scan(&targets, format)?;
        }
        Commands::Rules => rules()?,
    }

    Ok(())
}
