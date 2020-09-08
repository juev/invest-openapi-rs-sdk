#![allow(dead_code)]

use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::header;
use reqwest::Url;
use serde_json::json;
use serde_json::Value;

use self::types::*;

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

    pub fn instrument_by_ticker(&self, ticker: &str) -> Result<Instruments> {
        let path = format!(
            "{}/market/search/by-ticker?ticker={}",
            &self.api_url, ticker
        );
        let value = &self.do_request(path)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn currencies(&self) -> Result<Instruments> {
        let path = format!("{}/market/currencies", &self.api_url);
        let value = &self.do_request(path)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn etfs(&self) -> Result<Instruments> {
        let path = format!("{}/market/etfs", &self.api_url);
        let value = &self.do_request(path)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn bonds(&self) -> Result<Instruments> {
        let path = format!("{}/market/bonds", &self.api_url);
        let value = &self.do_request(path)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn stocks(&self) -> Result<Instruments> {
        let path = format!("{}/market/stocks", &self.api_url);
        let value = &self.do_request(path)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn operations(
        &self,
        account_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        figi: &str,
    ) -> Result<Operations> {
        let path = format!("{}/operations", &self.api_url);
        let mut url = Url::parse(&*path)?;
        url.query_pairs_mut()
            .clear()
            .append_pair("from", from.to_rfc3339().as_str())
            .append_pair("to", to.to_rfc3339().as_str());
        if figi != "" {
            url.query_pairs_mut().append_pair("figi", figi);
        }
        if account_id != DEFAULT_ACCOUNT.as_str() {
            url.query_pairs_mut()
                .append_pair("brokerAccountId", account_id);
        }
        println!("url: \n{:?}", url);
        let value = &self.do_request(url.to_string())?;
        println!("value: \n{:?}", value);
        let v: Operations = serde_json::from_value(json!(value))?;
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
