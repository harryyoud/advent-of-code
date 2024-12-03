pub mod paragraphs;
pub mod vector;

use std::{env, path::PathBuf};

use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest::{blocking::{ClientBuilder, Client}, header};

lazy_static! {
    static ref ENVIRONMENT: Result<PathBuf, dotenv::Error> = {
        dotenv()
    };
}

fn get_token_from_env() -> Result<String, env::VarError> {
    let _ = ENVIRONMENT.as_ref().ok();
    env::var("AOC_TOKEN")
}

fn get_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", header::HeaderValue::from_str(
        &get_token_from_env().expect("AOC_TOKEN must be specified as environment variable or in .env")
    ).unwrap());
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_input_year(year: usize, day: usize) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let x = get_client().get(url).send().unwrap();
    if !x.status().is_success() {
        panic!("Request error: {}: {}", x.status().as_str(), x.text().unwrap().trim())
    }
    x.text().unwrap().trim().to_string()
}