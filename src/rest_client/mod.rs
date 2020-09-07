#![allow(dead_code)]

use std::time::Duration;

use lazy_static::lazy_static;
use reqwest::Url;
use reqwest::{header, Error};

use self::types::Instrument;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;

mod types;

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

    pub fn new_sandbox(token: String) -> Self {
        Self {
            token,
            api_url: format!("{}/sandbox", RESTAPIURL.to_string()),
        }
    }

    pub fn instrument_by_figi(&self, figi: &str) -> Result<Instrument> {
        let path = format!("{}/market/search/by-figi?figi={}", &self.api_url, figi);
        let value = &self.do_request(path)?;
        let v: Instrument = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    fn do_request(&self, path: String) -> Result<Value> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let res = client
            .get(Url::parse(&path).unwrap())
            .bearer_auth(&self.token)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .send()
            .unwrap();
        let body = res.text()?;
        let json: Value = serde_json::from_str(&*body)?;
        let payload: Value = json!(json.get("payload"));
        Ok(payload)
    }
}
