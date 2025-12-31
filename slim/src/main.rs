use anyhow::Result;
use clap::Parser;

use slim::Slimmer;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
enum CargoCommand {
    Slim(Args),
}

/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
#[derive(clap::Args, Debug)]
struct Args {
    #[arg(long)]
    /// Don't clean anything just show what you would clean
    dry_run: bool,

    #[arg(default_value = ".")]
    /// Path to search for Rust projects
    paths: Vec<String>,
}

fn main() -> Result<()> {
    let CargoCommand::Slim(args) = CargoCommand::parse();
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
