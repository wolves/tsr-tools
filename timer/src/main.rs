use std::{env, process::Command, time::Instant};

use anyhow::Result;
use clap::{CommandFactory, Parser};

#[derive(Parser)]
struct Args {
    #[arg(short, long, required = true)]
    cmd: String,
}

fn main() -> Result<()> {
    if env::args().count() < 2 {
        Args::command().print_long_help();
        return Ok(());
    }
    let args = Args::parse();

    let start = Instant::now();
    let mut cmd = Command::new(args.cmd);
    let out = cmd.output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    println!("output: {stdout}");
    println!("That took {:?}", start.elapsed());
    Ok(())
}
