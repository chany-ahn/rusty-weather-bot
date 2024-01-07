# Rusty Weather Bot

## Description

This Discord Weather Bot, developed in Rust, provides real-time weather updates and weekly forecasts directly in your Discord server. It's designed to offer quick and accurate weather information with an easy-to-use interface by utilizing Discord's built in slash commands.

## Features

- Current Weather: Get the latest weather information including temperature, humidity, wind speed, and more for any specified location.
- Weekly Forecast: Displays weather forecasts for the upcoming week, giving you a comprehensive look at expected conditions.

## Dependencies

- Rust
- [WeatherAPI](https://www.weatherapi.com/) token
- Discord Bot token

## Installation

- Clone the repository to your local machine.
- Add your Discord Bot and WeatherAPI token to the `run_bot` file.
- Use `./run_bot` in your terminal to run the bot.

## Usage

The currently provided slash commands are:

- `/current_weather CITY_NAME`
- `/weekly_weather CITY_NAME`
