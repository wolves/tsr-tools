use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::env::{self};
use weather::Weatherstack;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    #[arg(
        short,
        long,
        env = "WEATHERSTACK_API_KEY",
        required = true
    )]
    /// Weatherstack API key
    api_key: String,
    #[arg(short, long)]
    /// Report temperatures in Farenheit
    farenheit: bool,
    #[arg(required = true)]
    /// Example: "London,UK"
    location: Vec<String>,
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
    println!(
        "{} {}",
        weather.summary,
        if args.farenheit {
            format!("{:.1}°F", weather.temperature.as_farenheit())
        } else {
            format!("{:.1}°C", weather.temperature.as_celsius())
        }
    );
    Ok(())
}
