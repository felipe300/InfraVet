use anyhow::Result;
use clap::Parser;

use crate::cli::Command;

mod analyzers;
mod cli;
mod commands;
mod core;
mod models;
mod utils;

use cli::Args;
use commands::scan;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Scan { file_type } => scan(file_type)?,
    }

    Ok(())
}
