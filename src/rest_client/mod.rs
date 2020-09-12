#![allow(dead_code)]

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use url::*;
use crate::*;

pub struct RestClient {
    token: String,
    api_url: Url,
}

impl RestClient {

    pub fn new(token: String) -> Self {
        Self {
            token,
            api_url: Url::parse("https://api-invest.tinkoff.ru/openapi/").unwrap(),
        }
    }

    pub fn new_sandbox(token: String) -> Self {
        Self {
            token,
            api_url: Url::parse("https://api-invest.tinkoff.ru/openapi/sandbox/").unwrap(),
        }
    }

    pub fn instrument_by_figi(&self, figi: &str) -> Result<Instrument> {
        let mut base = (&self).api_url.join("market/search/by-figi")?;
        base.query_pairs_mut()
            .clear()
            .append_pair("figi", figi)
            .finish();
        let value = &self.get_request(&base)?;
        let v: Instrument = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn instrument_by_ticker(&self, ticker: &str) -> Result<Instruments> {
        let mut base = (&self).api_url.join("market/search/by-ticker")?;
        base.query_pairs_mut()
            .clear()
            .append_pair("ticker", ticker)
            .finish();
        let value = &self.get_request(&base)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn currencies(&self) -> Result<Instruments> {
        let url = (&self).api_url.join("market/currencies")?;
        let value = &self.get_request(&url)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn etfs(&self) -> Result<Instruments> {
        let url = (&self).api_url.join("market/etfs")?;
        let value = &self.get_request(&url)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn bonds(&self) -> Result<Instruments> {
        let url = (&self).api_url.join("market/bonds")?;
        let value = &self.get_request(&url)?;
        let v: Instruments = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn stocks(&self) -> Result<Instruments> {
        let url = (&self).api_url.join("market/stocks")?;
        let value = &self.get_request(&url)?;
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
        let mut url = (&self).api_url.join("operations")?;
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
        let value = &self.get_request(&url)?;
        let v: Operations = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn portfolio(&self, account_id: &str) -> Result<Portfolio> {
        let positions = (&self).positions_portfolio(account_id)?;
        let currencies = (&self).currencies_portfolio(account_id)?;
        let portfolio = Portfolio {
            currencies,
            positions,
        };
        Ok(portfolio)
    }

    pub fn positions_portfolio(&self, account_id: &str) -> Result<PositionBalances> {
        let mut url = (&self).api_url.join("portfolio")?;
        if account_id != DEFAULT_ACCOUNT.as_str() {
            url.query_pairs_mut()
                .clear()
                .append_pair("brokerAccountId", account_id);
        }
        let value = &self.get_request(&url)?;
        let v: PositionBalances = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn currencies_portfolio(&self, account_id: &str) -> Result<CurrencyBalances> {
        let mut url = (&self).api_url.join("portfolio/currencies")?;
        if account_id != DEFAULT_ACCOUNT.as_str() {
            url.query_pairs_mut()
                .clear()
                .append_pair("brokerAccountId", account_id);
        }
        let value = &self.get_request(&url)?;
        let v: CurrencyBalances = serde_json::from_value(json!(value))?;
        Ok(v)
    }

    pub fn order_cancel(&self, account_id: &str, id: &str) -> Result<()> {
        let mut url = (&self).api_url.join("orders/cancel")?;
        url.query_pairs_mut()
            .clear()
            .append_pair("orderId", id);
        if account_id != DEFAULT_ACCOUNT.as_str() {
            url.query_pairs_mut()
                .clear()
                .append_pair("brokerAccountId", account_id);
        }
        (&self).post_request(url, "".to_string())?;
        Ok(())
    }

    pub fn limit_order(
        &self,
        account_id: &str,
        figi: &str,
        lots: i64,
        operation: OperationType,
        price: f64,
    ) -> Result<PlacedOrder> {
        let mut url = (&self).api_url.join("orders/limit-order")?;
        url.query_pairs_mut()
            .clear()
            .append_pair("figi", figi);
        if account_id != DEFAULT_ACCOUNT.as_str() {
            url.query_pairs_mut()
                .clear()
                .append_pair("brokerAccountId", account_id);
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct Body {
            lots: i64,
            operation: OperationType,
            price: f64,
        }
        let body = Body {
            lots,
            operation,
            price,
        };
        let body = serde_json::to_string(&body)?;
        // println!("{:?}", url);
        // println!("{:?}", body);
        let response = (&self).post_request(url, body)?;
        // println!("{:?}", response);
        let result = serde_json::from_value(json!(response))?;
        // println!("{:?}", result);
        Ok(result)
    }

    fn get_request(&self, url: &Url) -> Result<Value> {
        let response = attohttpc::get(&url.as_str())
            .header_append("Content-Type", "application/json")
            .header_append("Authorization", format!("Bearer {}", &self.token))
            .send()?;
        match response.error_for_status() {
            Ok(response) => {
                let json: Value = serde_json::from_str(&*response.text()?)?;
                Ok(json!(json.get("payload")))
            }
            Err(e) => Err(anyhow!("{:?}", e))
        }
    }

    fn post_request(&self, url: Url, body: String) -> Result<Value> {
        let response = attohttpc::post(&url.as_str())
            .header_append("Content-Type", "application/json")
            .header_append("Authorization", format!("Bearer {}", &self.token))
            .json(&body)?
            .send()?;
        match response.error_for_status() {
            Ok(response) => {
                let json: Value = serde_json::from_str(&*response.text()?)?;
                Ok(json!(json.get("payload")))
            }
            Err(e) => Err(anyhow!("{:?}", e))
        }
    }
}
