use std::{env, process::Command, time::Instant};

use anyhow::Result;
use clap::{CommandFactory, Parser};

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    program: String,

    args: Vec<String>,
}

fn main() -> Result<()> {
    if env::args().count() < 2 {
        Args::command().print_long_help();
        return Ok(());
    }
    let args = Args::parse();

    let report = timer::time(&args.program, &args.args)?;

    println!("{}", report.stdout);
    println!("{}", report.stderr);
    println!("{:.1?}", report.elapsed);
    Ok(())
}
