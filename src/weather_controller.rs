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

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.region == other.region && self.country == other.country
    }
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

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.icon == other.icon
    }
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

impl PartialEq for CurrentWeather {
    fn eq(&self, other: &Self) -> bool {
        self.temp_c == other.temp_c &&
        self.feelslike_c == other.feelslike_c &&
        self.wind_kph == other.wind_kph &&
        self.wind_dir == other.wind_dir &&
        self.precip_mm == other.precip_mm &&
        self.condition == other.condition
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDayWeather {
    maxtemp_c: f32,
    mintemp_c: f32,
    totalprecip_mm: f32,
    condition: Condition
}

impl PartialEq for ForecastDayWeather {
    fn eq(&self, other: &Self) -> bool {
        self.maxtemp_c == other.maxtemp_c &&
        self.mintemp_c == other.mintemp_c &&
        self.totalprecip_mm == other.totalprecip_mm
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
    date: String,
    day: ForecastDayWeather,

}

impl PartialEq for ForecastDay {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date &&
        self.day == other.day 
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    location: Location,
    current: CurrentWeather,
    forecast: Option<Forecast>,
}

impl PartialEq for WeatherInfo {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location &&
        self.current == other.current &&
        self.forecast == other.forecast
    }
}

impl WeatherInfo {
    pub fn display_weather_info(&self) -> String {
        let mut display_string = format!(
            "# {}\n## Today:\nTemp: {}, Feels Like: {}\n",
            self.location.get_formatted_location(), 
            self.current.temp_c,
            self.current.feelslike_c,
        );
        
        if let Some(forecast) = &self.forecast {
            let forcast_days = &forecast.forecastday;
            display_string.push_str("## Next 6 days:\n");
            for day in forcast_days {
                let cur_day = format!("* {}\n * Max Temp: {}\n * Min Temp: {}\n * Projected Precipition: {}\n", 
                    day.date, 
                    day.day.maxtemp_c, 
                    day.day.mintemp_c, 
                    day.day.totalprecip_mm);
                display_string.push_str(&cur_day);
            }

        }
        display_string
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


#[async_trait]
pub trait WeatherController{
    fn new(base_url: &str, api_key: &str, city_name: &str) -> Self;
    async fn get_weather(&self) -> Result<WeatherInfo>;
}

pub struct CurrentWeatherController {
    base_url: String,
    api_key: String,
    city_name: String,
}


#[async_trait]
impl WeatherController for CurrentWeatherController {

    fn new(base_url: &str, api_key: &str, city_name: &str) -> CurrentWeatherController {
        CurrentWeatherController { base_url: String::from(base_url), api_key: String::from(api_key), city_name: String::from(city_name) } 
    }

    async fn get_weather(&self) -> Result<WeatherInfo> {
        let req_url = format!("{}{}?key={}&q={}&aqi=yes", self.base_url, "/current.json", self.api_key, self.city_name);
        println!("This is the request_url: {req_url}");

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
    base_url: String,
    api_key: String,
    city_name: String,
}

#[async_trait]
impl WeatherController for WeeklyWeatherController {

    fn new(base_url: &str, api_key: &str, city_name: &str) -> WeeklyWeatherController {
        WeeklyWeatherController { base_url: String::from(base_url), api_key: String::from(api_key), city_name: String::from(city_name) } 
    } 


    async fn get_weather(&self) -> Result<WeatherInfo> {
        let req_url = format!("{}{}?key={}&q={}&aqi=no&days={}&alerts=no", self.base_url, "/forecast.json", self.api_key, self.city_name, 7);
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

