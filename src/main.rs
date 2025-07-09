// src/main.rs
/*
 * Main executable for ErcNftMetadataIndexer
 */

use clap::Parser;
use ercnftmetadataindexer::{Result, run};

#[derive(Parser)]
#[command(version, about = "ErcNftMetadataIndexer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
