use anyhow::{Result, bail};
use std::env;

use count::count_in_path;

fn main() -> Result<()> {
    let mut word_mode = false;
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count <FILE>...");
    }

    for arg in args {
        if arg == "-w" {
            word_mode = true;
            continue;
        }
        let count = count_in_path(&arg)?;

        if word_mode {
            println!("{arg}: {} words", count.words);
        } else {
            println!("{arg}: {} lines", count.lines);
        }
    }
    Ok(())
}
