#![allow(dead_code)]
use lazy_static::lazy_static;
use std::io::Read;
mod types;
use self::types::Instrument;
use reqwest::Url;

lazy_static! {
    static ref RESTAPIURL: String = String::from("https://api-invest.tinkoff.ru/openapi");
}
pub struct RestClient {
    // httpClient *http.Client
    token: String,
    api_url: String,
}

impl RestClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            api_url: RESTAPIURL.to_string(),
        }
    }

    pub fn instrument_by_figi(&self, figi: &str) -> Instrument {
        let path = format!(
            "{}/market/search/by-figi?figi={}",
            RESTAPIURL.to_string(),
            figi
        );
        println!("path = {}", path);
        println!("token: {}", &self.token);

        println!("URL: {}", Url::parse(&path).unwrap());
        let client = reqwest::blocking::Client::new();
        let mut res = client
            .get(Url::parse(&path).unwrap())
            .bearer_auth(&self.token)
            .send()
            .unwrap();
        println!("{:?}", res);
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{:?}", body);
        let v: Instrument = serde_json::from_str(&body[..]).unwrap();
        v
    }
}
