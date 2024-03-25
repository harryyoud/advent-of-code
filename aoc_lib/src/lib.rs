use std::{env, path::PathBuf};
use std::str::Lines;

use dotenv::dotenv;
use itertools::{Batching, Itertools};
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

pub type Paragraph<'a> = Batching<Lines<'a>, fn(&mut Lines<'a>) -> Option<Vec<&'a str>>>;

pub trait Paragraphs {
    fn paragraphs<'a>(&'a self) -> Paragraph<'a>;
}

impl Paragraphs for &str {
    fn paragraphs<'a>(&'a self) -> Paragraph<'a> {
        fn inner_batch<'a>(lines: &mut Lines<'a>) -> Option<Vec<&'a str>> {
            let out = lines.take_while(|line| !line.is_empty()).collect_vec();
            (!out.is_empty()).then(|| out)
        }
        self.lines().batching(inner_batch as fn(&mut Lines<'a>) -> Option<Vec<&'a str>>)
    }
}

impl Paragraphs for String {
    fn paragraphs<'a>(&'a self) -> Paragraph<'a> {
        fn inner_batch<'a>(lines: &mut Lines<'a>) -> Option<Vec<&'a str>> {
            let out = lines.take_while(|line| !line.is_empty()).collect_vec();
            (!out.is_empty()).then(|| out)
        }
        self.lines().batching(inner_batch as fn(&mut Lines<'a>) -> Option<Vec<&'a str>>)
    }
}