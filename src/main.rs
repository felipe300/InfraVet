use anyhow::Result;
use clap::Parser;

use crate::cli::Commands;

mod analyzers;
mod cli;
mod commands;
mod core;
mod models;
mod utils;

use cli::Args;
use commands::{rules, scan};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Scan { file_type, format } => scan(file_type, format)?,
        Commands::Rules => rules()?,
    }

    Ok(())
}
