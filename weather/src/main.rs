use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::env::{self};
use weather::Weatherstack;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    #[arg(required = true)]
    /// Example: "London,UK"
    location: Vec<String>,

    #[arg(
        short,
        long,
        env = "WEATHERSTACK_API_KEY",
        required = true
    )]
    /// Weatherstack API key
    api_key: String,
}

fn main() -> Result<()> {
    if env::args().count() < 2 {
        Args::command().print_long_help()?;
        return Ok(());
    }
    let args = Args::parse();
    let location = args.location.join(" ");
    let ws = Weatherstack::new(&args.api_key);
    let weather = ws.get_weather(&location)?;
    println!("{weather}");
    Ok(())
}
