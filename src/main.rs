use std::io;
use serde::Deserialize;
use colored::*;
use chrono::{ Local };

#[derive(Deserialize)]
struct WeatherResponse {
    name: String,
    main: MainData,
    weather: Vec<Weather>,
    wind: Wind,
    sys: Sys,
}

#[derive(Deserialize)]
struct Sys {
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[derive(Deserialize)]
struct MainData {
    temp: f64,
    feels_like: f64,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, String> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        city,
        api_key
    );

    let response = match reqwest::blocking::get(&url) {
        Ok(res) => res,
        Err(e) => {
            return Err(format!("Network error: {}", e));
        }
    };

    if !response.status().is_success() {
        return match response.json::<ApiError>() {
            Ok(api_error) => Err(format!("API Error: {}", api_error.message)),
            Err(_) => Err("Unknown API error".to_string()),
        };
    }

    match response.json::<WeatherResponse>() {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Parsing error: {}", e)),
    }
}

fn main() {
    println!("{}", "Welcome to Simple Weather App!".bright_blue());
    let api_key = "20067aa8a70ff72621bf26227a324cc4";

    loop {
        println!("{}", "\nEnter a city name (or 'quit' to exit):".bright_cyan());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let city = input.trim();
        if city.eq_ignore_ascii_case("quit") {
            break;
        }

        match get_weather(city, api_key) {
            Ok(data) => {
                println!(
                    "{} Today feels like {}°C in {} {}",
                    Local::now().format("%A, %B %d %Y %H:%M:%S"),
                    data.main.feels_like,
                    data.name,
                    data.sys.country
                );
                println!("Recoreded at {}", data.sys.sunrise);
                println!("Sunset at {}", data.sys.sunset);
                println!("Recoreded temperature: {:.1}°C", data.main.temp);
                println!("Conditions: {}", data.weather[0].description.yellow());
                println!("Wind Speed: {:.1} m/s", data.wind.speed);
            }
            Err(e) => println!("{} {}", "Error:".red(), e),
        }
    }
}
