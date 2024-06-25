extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

async fn index() -> HttpResponse {
    let api_key =
        env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY is not set in .env");
    let city = "Montreal";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = Client::new().get(&url).send().await.unwrap();

    match response.json::<WeatherResponse>().await {
        Ok(weather_data) => {
            let temperature = weather_data.main.temp;
            let city_name = weather_data.name;
            HttpResponse::Ok().body(format!("Weather in {}: {:.1} °C", city_name, temperature))
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to parse weather data"),
    }
}

fn print_env_file() {
    let file = File::open(".env").expect("Could not open .env file");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.expect("Could not read line from .env file"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_env_file(); // Add this line
    dotenv().ok();
    println!("API Key: {:?}", env::var("OPENWEATHERMAP_API_KEY"));

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
