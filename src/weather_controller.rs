use poise::async_trait;
use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;

use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String 
}

impl Location {
    pub fn get_formatted_location(&self) -> String {
        format!("{}, {}, {}", self.name, self.region, self.country)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    text: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    temp_c: f32,
    feelslike_c: f32,
    wind_kph: f32,
    wind_dir: String,
    precip_mm: f32,
    condition: Condition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDayWeather {
    maxtemp_c: f32,
    mintemp_c: f32,
    totalprecip_mm: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
    date: String,
    day: ForecastDayWeather,
    condition: Condition

}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    location: Location,
    current: CurrentWeather,
    forecast: Option<Vec<ForecastDay>>
}

impl WeatherInfo {
    pub fn display_weather_info(&self) -> String {
        format!(
            r#"{}
Temp: {}, Feels Like: {}
        "#,
            self.location.get_formatted_location(), 
            self.current.temp_c,
            self.current.feelslike_c,
        )
    }
}

pub struct ApiUtils {}

impl ApiUtils {
    pub async fn send_request(req_url: &str) -> Result<String> {
        
        let client = reqwest::Client::new();
        let resp = client
            .get(req_url)
            .send()
            .await?;

        if resp.status().is_success() {
        Ok(resp.text().await?)
        } else {
            Err(Box::new(ApiError {}))
        }
    }

}

#[derive(Debug)]
pub struct ApiError;

impl Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error retrieving information from you API!")
    }
}

const BASE_URL: &str = "http://api.weatherapi.com/v1";

#[async_trait]
pub trait WeatherController{
    fn new(api_key: &str, city_name: &str) -> Self;
    async fn get_weather(&self) -> Result<WeatherInfo>;
}

pub struct CurrentWeatherController {
    api_key: String,
    city_name: String,
}

#[async_trait]
impl WeatherController for CurrentWeatherController {

    fn new(api_key: &str, city_name: &str) -> CurrentWeatherController {
        CurrentWeatherController { api_key: String::from(api_key), city_name: String::from(city_name) } 
    } 


    async fn get_weather(&self) -> Result<WeatherInfo> {
        let req_url = format!("{}/{}?key={}&q={}&aqi=yes", BASE_URL, "current.json", self.api_key, self.city_name);

        let resp = ApiUtils::send_request(&req_url).await?;

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
    
}

pub struct WeeklyWeatherController {
    api_key: String,
    city_name: String,
}

#[async_trait]
impl WeatherController for WeeklyWeatherController {

    fn new(api_key: &str, city_name: &str) -> WeeklyWeatherController {
        WeeklyWeatherController { api_key: String::from(api_key), city_name: String::from(city_name) } 
    } 


    async fn get_weather(&self) -> Result<WeatherInfo> {
        let req_url = format!("{}/{}?key={}&q={}&aqi=no&days={}&alerts=no", BASE_URL, "forecast.json", self.city_name, self.api_key, 7);
        let resp = ApiUtils::send_request(&req_url).await?;

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
    
}

