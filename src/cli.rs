use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Subcmd,
}

#[derive(Subcommand, Debug)]
pub enum Subcmd {
    /// Search for a file recursively
    Search {
        #[clap(short = 'f')]
        #[clap(long = "filename")]
        filename: String,
    },
}
