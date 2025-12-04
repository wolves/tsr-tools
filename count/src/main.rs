use anyhow::{Context, Result, bail};
use std::{env, fs::File, io::BufReader};

use count::count_lines;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count <FILE>...");
    }

    for path in args {
        let file =
            File::open(&path).with_context(|| path.clone())?;
        let file = BufReader::new(file);
        let lines =
            count_lines(file).with_context(|| path.clone())?;

        println!("{path}: {lines} lines");
    }
    Ok(())
}
