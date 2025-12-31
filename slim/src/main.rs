use anyhow::Result;
use clap::Parser;

use slim::Slimmer;

/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
#[derive(Parser)]
struct Args {
    #[arg(long)]
    /// Don't clean anything just show what you would clean
    dry_run: bool,

    #[arg(default_value = ".")]
    /// Path to search for Rust projects
    paths: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut slimmer = Slimmer::new();
    if args.dry_run {
        slimmer.dry_run = true;
    }
    for path in &args.paths {
        let output = slimmer.slim(path)?;
        print!("{output}");
    }
    Ok(())
}
