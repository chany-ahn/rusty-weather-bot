use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;

use std::error::Error;
use std::fmt;

const BASE_URL: &str = "http://api.weatherapi.com/v1";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    country: String,
    localtime: String 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirQuality {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    temp_c: f32,
    feelslike_c: f32,
    wind_kph: f32,
    wind_dir: String,
    precip_mm: f32,
    humidity: u8,
    uv: f32,
    air_quality: AirQuality,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    location: Location,
    current: CurrentWeather,
}

impl WeatherInfo {
    pub fn display_weather(&self) -> String {
        format!("The current weather is {} deg C", self.current.temp_c) 
    }
}

#[derive(Debug)]
pub struct WeatherApiError;

impl Error for WeatherApiError {}

impl fmt::Display for WeatherApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error retrieving information from WeatherApi!")
    }
}

pub struct WeatherApiController {
    api_key: String,
}

impl WeatherApiController {
    pub fn new(api_key: &str) -> WeatherApiController {
        WeatherApiController{
            api_key: String::from(api_key)
        }
    } 

    async fn send_weather_api_request(req_url: &str) -> Result<String> {
        
        let client = reqwest::Client::new();
        let resp = client
            .get(req_url)
            .send()
            .await?;

        if resp.status().is_success() {
        Ok(resp.text().await?)
        } else {
            Err(Box::new(WeatherApiError {}))
        }
    }

    pub async fn get_current_weather(&self, city_name: &str) -> Result<WeatherInfo> {
        let req_url = format!("{}/{}?key={}&q={}&aqi=yes", BASE_URL, "current.json", self.api_key, city_name);

        let resp = Self::send_weather_api_request(&req_url).await?;

        let parsed_weather_info = serde_json::from_str(&resp);

        match parsed_weather_info {
            Ok(weather_info) => {
                println!("this is the current weather parsed json: {:?}", weather_info);
                Ok(weather_info)
            },
            Err(e) => {
                println!("Unable to parse the weekly weather info properly. Error thrown: {}", e);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Parsing JSON failed",
                )))
            }
        }
    }

    pub async fn get_weekly_weather(&self, city_name: &str, days: u8) -> Result<WeatherInfo> {
        let req_url = format!("{}/{}?key={}&q={}&aqi=no&days={}&alerts=no", BASE_URL, "forecast.json", city_name, self.api_key, days);

        let resp = Self::send_weather_api_request(&req_url).await?;
    
        let parsed_weather_info = serde_json::from_str(&resp);

        match parsed_weather_info {
            Ok(weather_info) => {
                println!("this is the next {} day forecast data parsed json: {:?}", days, weather_info);
                Ok(weather_info)
            },
            Err(e) => {
                println!("Unable to parse the weekly weather info properly. Error thrown: {}", e);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Parsing JSON failed",
                )))
            }
        }

    }
}

