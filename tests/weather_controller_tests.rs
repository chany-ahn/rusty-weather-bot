#[cfg(test)]
mod weather_controller_tests {
    use weather_controller::{WeatherController,CurrentWeatherController, WeeklyWeatherController};
    #[tokio::test]
    async fn test_get_current_weather() {
        let body = r#"{
    "location": {
        "name": "Toronto",
        "region": "Ontario",
        "country": "Canada",
        "localtime": "2023-11-04 21:39"
    },
    "current": {
        "temp_c": 5.0,
        "is_day": 0,
        "condition": {
            "text": "Partly cloudy",
            "icon": "//cdn.weatherapi.com/weather/64x64/night/116.png",
            "code": 1003
        },
        "wind_mph": 21.7,
        "wind_kph": 34.9,
        "wind_degree": 60,
        "wind_dir": "ENE",
        "pressure_mb": 1027.0,
        "pressure_in": 30.34,
        "precip_mm": 0.0,
        "precip_in": 0.0,
        "humidity": 87,
        "cloud": 50,
        "feelslike_c": 0.9,
        "feelslike_f": 33.7,
        "gust_mph": 26.2,
        "gust_kph": 42.1
    }
}
"#;
        let mut server = mockito::Server::new();

        let url = "/current.json?key=fake_key&q=Toronto&aqi=yes";

        let mock = server
            .mock("GET", url)
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(body)
            .create();
        
        let weather_controller = CurrentWeatherController::new(&server.url(), "fake_key", "Toronto");
        let weather_result = weather_controller.get_weather().await;

        mock.assert();
        
        let expected = serde_json::from_str::<weather_controller::WeatherInfo>(body);
        match (expected, weather_result) {
            (Ok(expected_weather_info), Ok(weather_info)) => {
                assert_eq!(expected_weather_info, weather_info);
            },
            _ => {
                assert!(false);
            }
        }

    }

    #[tokio::test]
    async fn test_weekly_weather_controller() {
        let body = r#"{
    "location": {
        "name": "Toronto",
        "region": "Ontario",
        "country": "Canada",
        "localtime": "2023-11-04 21:39"
    },
    "current": {
        "temp_c": 5.0,
        "is_day": 0,
        "condition": {
            "text": "Partly cloudy",
            "icon": "//cdn.weatherapi.com/weather/64x64/night/116.png",
            "code": 1003
        },
        "wind_mph": 21.7,
        "wind_kph": 34.9,
        "wind_degree": 60,
        "wind_dir": "ENE",
        "pressure_mb": 1027.0,
        "pressure_in": 30.34,
        "precip_mm": 0.0,
        "precip_in": 0.0,
        "humidity": 87,
        "cloud": 50,
        "feelslike_c": 0.9,
        "feelslike_f": 33.7,
        "gust_mph": 26.2,
        "gust_kph": 42.1
    },
    "forecast": {
        "forecastday": [
            {
                "date": "2023-12-19",
                "day": {
                    "maxtemp_c": 11.6,
                    "mintemp_c": 6.3,
                    "avgtemp_c": 10.1,
                    "totalprecip_mm": 10.65,
                    "condition": {
                        "text": "Moderate rain",
                        "icon": "//cdn.weatherapi.com/weather/64x64/day/302.png",
                        "code": 1189
                    },
                    "uv": 3.0
                }
            }
        ]
    }
}
"#;
        let mut server = mockito::Server::new();

        let url = "/forecast.json?key=fake_key&q=Toronto&aqi=no&days=7&alerts=no";

        let mock = server
            .mock("GET", url)
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(body)
            .create();
        
        let weather_controller = WeeklyWeatherController::new(&server.url(), "fake_key", "Toronto");
        let weather_result = weather_controller.get_weather().await;

        mock.assert();
        
        let expected = serde_json::from_str::<weather_controller::WeatherInfo>(body);
        match (expected, weather_result) {
            (Ok(expected_weather_info), Ok(weather_info)) => {
                assert_eq!(expected_weather_info, weather_info);
            },
            _ => {
                assert!(false);
            }
        }
    }
}

#[cfg(test)]
mod api_utils_test {

    #[tokio::test]
    async fn test_send_request() {
        let body = r#"{
    "location": {
        "name": "Toronto",
        "region": "Ontario",
        "country": "Canada",
        "localtime": "2023-11-04 21:39"
    },
    "current": {
        "temp_c": 5.0,
        "is_day": 0,
        "condition": {
            "text": "Partly cloudy",
            "icon": "//cdn.weatherapi.com/weather/64x64/night/116.png",
            "code": 1003
        },
        "wind_mph": 21.7,
        "wind_kph": 34.9,
        "wind_degree": 60,
        "wind_dir": "ENE",
        "pressure_mb": 1027.0,
        "pressure_in": 30.34,
        "precip_mm": 0.0,
        "precip_in": 0.0,
        "humidity": 87,
        "cloud": 50,
        "feelslike_c": 0.9,
        "feelslike_f": 33.7,
        "gust_mph": 26.2,
        "gust_kph": 42.1
    }
}
"#;
        let mut server = mockito::Server::new();

        let url = "/weatherapimock";

        let mock = server
            .mock("GET", url)
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body(body)
            .create();

        let req_url = format!("{}{}", server.url(), url);
        let response = weather_controller::ApiUtils::send_request(&req_url).await;
        
        mock.assert();

        match response {
            Ok(resp) => {
                assert_eq!(body, resp);
            },
            Err(e) => {
                println!("{e}");
                assert!(false);
            },
        }
    }
}

#[cfg(test)]
mod weather_info_tests {
    use weather_controller::WeatherInfo;
    
    #[test]
    fn test_display_weather() {
        
        let body = r#"{
    "location": {
        "name": "Toronto",
        "region": "Ontario",
        "country": "Canada",
        "localtime": "2023-11-04 21:39"
    },
    "current": {
        "temp_c": 5.1,
        "is_day": 0,
        "condition": {
            "text": "Partly cloudy",
            "icon": "//cdn.weatherapi.com/weather/64x64/night/116.png",
            "code": 1003
        },
        "wind_mph": 21.7,
        "wind_kph": 34.9,
        "wind_degree": 60,
        "wind_dir": "ENE",
        "pressure_mb": 1027.0,
        "pressure_in": 30.34,
        "precip_mm": 0.0,
        "precip_in": 0.0,
        "humidity": 87,
        "cloud": 50,
        "feelslike_c": 0.9,
        "feelslike_f": 33.7,
        "gust_mph": 26.2,
        "gust_kph": 42.1
    }
}
"#;
        let parsed_weather_info: Result<WeatherInfo, serde_json::Error> = serde_json::from_str(body);
        match parsed_weather_info {
            Ok(weather_info) => {
                let expected_weather_display_str = format!(
            "# Toronto, Ontario, Canada\n## Today:\nTemp: 5.1, Feels Like: 0.9\n");
                assert_eq!(expected_weather_display_str, weather_info.display_weather_info());
            },
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    } 
}
