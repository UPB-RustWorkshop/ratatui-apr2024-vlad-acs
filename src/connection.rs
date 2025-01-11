use core::panic;
use std::{collections::HashMap, error, string};

use chrono::{DateTime, Local};
use futures::TryFutureExt;
use reqwest::Response;

pub struct CityInfo {
    // TODO: define elements in the structure
    pub name: String,
    pub weather: String,
    pub temperature: u32
}

// Method that is handling the request to the openweather api,
// parsing the response
//
// IP: 34.116.205.113
// Port: 3000

/// Returns names of all cities
pub fn get_all_cities() -> Vec<String> {
    match reqwest::blocking::get("http://34.116.205.113:3000/cities") {
        Ok(response) => {
            if response.status().is_success() {
                return response.json().into_iter().collect::<Vec<String>>();
            } else {
                panic!();
            }
        }
        Err(error) => {
            panic!();
        }
    }
}

/// Returns weather details about a certain city
pub fn get_city_info(city: &String) -> Option<CityInfo> {
    match reqwest::blocking::Client::new().post("http://34.116.205.113:3000/cities/city_weather")
        .form(&HashMap::from([("City", &city)]))
        .send() {
        Ok(response) => {
            // Check status code
            if response.status().is_success() {
                // Parse response
                return Option::Some(CityInfo {name: city.clone(), weather: "none".to_string(), temperature: 0});
            } else {
                return Option::None;
            }
        },
        Err(error) => {
            return Option::None;
        }
    }
}