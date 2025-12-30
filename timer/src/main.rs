use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let mut cmd = Command::new("cargo");
    let out = cmd.output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    println!("output: {stdout}");
    Ok(())
}
