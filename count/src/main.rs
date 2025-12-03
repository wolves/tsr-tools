use anyhow::{Context, Result};
use std::{env, fs::File, io::BufReader};

use count::count_lines;

fn main() -> Result<()> {
    for path in env::args().skip(1) {
        let file = File::open(&path)?;
        let file = BufReader::new(file);
        let lines = count_lines(file).context(path)?;

        println!("{lines} lines");
    }
    Ok(())
}
