#![allow(dead_code)]

use std::io::Read;
use std::time::Duration;

use lazy_static::lazy_static;
use reqwest::header;
use reqwest::Url;

use self::types::Instrument;
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

    pub fn instrument_by_figi(&self, figi: &str) -> Instrument {
        let path = format!("{}/market/search/by-figi?figi={}", &self.api_url, figi);
        let v: Instrument = serde_json::from_value(json!(&self.do_request(path))).unwrap();
        v
    }

    fn do_request(&self, path: String) -> Value {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let mut res = client
            .get(Url::parse(&path).unwrap())
            .bearer_auth(&self.token)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let root: Value = serde_json::from_str(&body[..]).unwrap();
        let payload: Value = json!(root.get("payload"));
        payload
    }
}
