# Weather Forecast Terminal Application

This is a simple terminal application written in Rust that fetches and displays the weather forecast for a specified city using the OpenWeatherMap API.

## Features

- Fetches current weather data for a specified city.
- Displays temperature, pressure, humidity, and weather conditions.
- Utilizes the OpenWeatherMap API.

## Prerequisites

- Rust and Cargo installed. You can install Rust and Cargo by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/weather_forecast.git
    cd weather_forecast
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Install the binary:

    ```sh
    sudo cp target/release/weather_forecast /usr/local/bin/weather_forecast
    ```

## Usage

To run the application, use the following command, replacing `<CITY>` with the desired city and `<API_KEY>` with your OpenWeatherMap API key:

```sh
weather_forecast <CITY> <API_KEY>
```

## Example

```sh
weather_forecast --city London --api-key your_api_key_here
```

```
Weather in London: 
Temperature: 15.0°C
Pressure: 1012 hPa
Humidity: 87%
Condition: clear sky
```
