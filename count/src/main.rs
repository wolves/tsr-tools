use anyhow::{Result, bail};
use std::env;

use count::count_in_path;

use clap::Parser;
use clap::arg;

#[derive(Parser)]
/// Counts lines or words in the specified files
struct Args {
    /// Counts words instead of lines
    #[arg(short, long)]
    words: bool,

    /// Files to be counted
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for path in args.files {
        let count = count_in_path(&path)?;
        if args.words {
            println!("{path}: {} words", count.words);
        } else {
            println!("{path}: {} lines", count.lines);
        }
    }
    Ok(())
}
