use std::env;

use anyhow::Result;

use weather::get_weather;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let location = args.join(" ");

    let api_key = env::var("WEATHERSTACK_API_KEY")?;
    let weather = get_weather(&location, &api_key)?;
    // let resp = reqwest::blocking::Client::new()
    //     .get("https://api.weatherstack.com/current")
    //     .query(&[("query", "London,UK"), ("access_key", &api_key)])
    //     .send()
    //     .unwrap();

    println!("{weather}");
    Ok(())
}
