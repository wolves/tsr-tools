use anyhow::{Context, Result, bail};
use std::env;

use count::count_lines_in_path;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count <FILE>...");
    }

    for path in args {
        let lines = count_lines_in_path(&path)
            .with_context(|| path.clone())?;

        println!("{path}: {lines} lines");
    }
    Ok(())
}
