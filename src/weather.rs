use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}

pub async fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(response)
}