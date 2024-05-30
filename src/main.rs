use axum::{
    extract::Query,
    response::sse::{Event, Sse},
    routing::get,
    Extension, Router,
};
use reqwest::Client;
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio::time::sleep;
use tokio_stream::StreamExt as _;
use async_stream::stream;
use axum::response::IntoResponse;

#[derive(Debug, Deserialize)]
struct WeatherInfo {
    city: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    humidity: u8,
}

async fn weather_info_handler(
    Query(params): Query<WeatherInfo>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    let city = params.city.clone();
    let stream = stream! {
        let api_key = "acae54b0816be24d5ae63bc90b9a30ce";
        loop {
            match get_weather_info(&client, &city, api_key).await {
                Ok(weather) => {
                    let data = format!("Temperature: {}°C, Humidity: {}%", weather.temp, weather.humidity);
                    yield Ok::<_, Infallible>(Event::default().data(data));
                }
                Err(err) => {
                    eprintln!("Error fetching weather data: {}", err);
                    yield Ok::<_, Infallible>(Event::default().data("Error fetching weather data".to_string()));
                }
            }
            sleep(Duration::from_secs(3)).await;
        }
    };
    Sse::new(stream)
}

async fn get_weather_info(client: &Client, city: &str, api_key: &str) -> Result<Main, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}", city, api_key);
    let res = client.get(&url).send().await?.json::<WeatherResponse>().await?;
    Ok(res.main)
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app = Router::new()
        .route("/weather-info", get(weather_info_handler))
        .layer(Extension(client));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}