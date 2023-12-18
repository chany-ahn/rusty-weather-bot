use weather_controller::*; 
use crate::Context;
use std::error::Error;
use std::env;
use std::fmt;

const WEATHER_API_URL: &str = "http://api.weatherapi.com/v1";

#[derive(Debug)]
pub struct EnvVariableError;

impl Error for EnvVariableError {}

impl fmt::Display for EnvVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Trouble getting an environment variable")
    }
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn todays_weather(ctx: Context<'_>, city: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let weather_api_env_var = env::var("WEATHER_API_KEY");
    let weather_api_controller;  

    match weather_api_env_var {
        Ok(key) => {
            weather_api_controller = CurrentWeatherController::new(WEATHER_API_URL, &key, &city);
        },
        Err(e) => {
            println!("Ran into error: {e}. Failed to get the WEATHER_API_KEY. Did you set it properly?");
            return Err(Box::new(EnvVariableError {}));
        }
    }

    let current_weather = weather_api_controller.get_weather().await?;
    
    ctx.say(current_weather.display_weather_info()).await?;

    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn weekly_weather(ctx: Context<'_>, city: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let weather_api_env_var = env::var("WEATHER_API_KEY");
    let weather_api_controller;  

    match weather_api_env_var {
        Ok(key) => {
            weather_api_controller = WeeklyWeatherController::new(WEATHER_API_URL, &key, &city);
        },
        Err(e) => {
            println!("Ran into error: {e}. Failed to get the WEATHER_API_KEY. Did you set it properly?");
            return Err(Box::new(EnvVariableError {}));
        }
    }

    let current_weather = weather_api_controller.get_weather().await?;
    
    ctx.say(current_weather.display_weather_info()).await?;

    Ok(())
}
