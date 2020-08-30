#![allow(dead_code)]
use lazy_static::lazy_static;

mod types;
use self::types::Instrument;
use std::io::Read;

lazy_static! {
    #[derive(Debug)]
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

    pub fn instrument_by_figi(figi: String) -> Instrument {
        let path = format!("{:?}/market/search/by-figi?figi={}", RESTAPIURL, figi);
        let mut res = reqwest::blocking::get(&path[..]).unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body);
        let v: Instrument = serde_json::from_str(&body[..]).unwrap();
        v
    }
}
