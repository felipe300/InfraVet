use anyhow::Result;
use clap::Parser;

use crate::cli::Command;

mod cli;
mod commands;
mod utils;

use cli::Args;
use commands::search;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Search { filename } => search(filename)?,
    }

    Ok(())
}
