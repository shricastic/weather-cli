use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn getWeatherInfo(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json()?;

    Ok(response_json)
}

fn display_weather(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temp = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_tmp_emoji(temp),
        temp,
        humidity,
        pressure,
        wind_speed
    );

    let weather_txt_color: ColoredString = match description.as_str() {
        "clear sky" => weather.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "fog" => weather.dimmed(),
        "rain" | "thunderstorm" | "snow" => weather.bright_cyan(),
        _ => weather.normal(),
    };

    println!("{}", weather_txt_color);
}

fn get_tmp_emoji(temp: f64) -> &'static str {
    if temp < 0.0 {
        "❄️"
    } else if temp < 10.0 {
        "☁️"
    } else if temp < 20.0 {
        "⛅"
    } else if temp < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather CLI ~ @Shricastic");

    loop {
        println!("{}", "Name of the city:".bright_green());

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        let api_key = "3842f9cddbb4cd4c29c7a3d094ca7c4f";

        match getWeatherInfo(city, country_code, api_key) {
            Ok(response) => display_weather(&response),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
