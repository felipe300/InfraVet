use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
// mod models;

use cli::{Args, Subcmd};
use commands::search;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcmd::Search { filename } => search(filename)?,
    }

    Ok(())
}
