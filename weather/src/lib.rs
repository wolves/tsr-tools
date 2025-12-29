use std::fmt::Display;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Weather {
    temperature: f64,
    summary: String,
}

impl Display for Weather {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn get_weather(
    location: &str,
    api_key: &str,
) -> Result<Weather> {
    Ok(Weather {
        temperature: 11.2,
        summary: "Rainy".into(),
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn get_weather_fn_returns_correct_weather_for_location() {
        let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
        let weather = get_weather("London,UK", &api_key).unwrap();
        assert_eq!(
            weather,
            Weather {
                temperature: 11.2,
                summary: "Sunny".into(),
            },
            "wrong weather"
        )
    }
}
