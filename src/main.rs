mod weather;

use weather::get_weather;
use clap::Parser;

/// A simple program to fetch and display weather forecast
#[derive(Parser)]
#[command(name = "weather_forecast")]
#[command(about = "Fetches weather information for a specified city using OpenWeatherMap API", long_about = None)]
struct Args {
    /// The city to fetch the weather for
    pub city: String,
    /// Your OpenWeatherMap API key
    pub api_key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Fetch the weather data
    match get_weather(&args.city, &args.api_key).await {
        Ok(weather) => {
            println!("Weather in {}: ", args.city);
            println!("Temperature: {}°C", weather.main.temp);
            println!("Pressure: {} hPa", weather.main.pressure);
            println!("Humidity: {}%", weather.main.humidity);
            for w in weather.weather {
                println!("Condition: {}", w.description);
            }
        }
        Err(e) => {
            eprintln!("Error fetching weather data: {}", e);
        }
    }
}