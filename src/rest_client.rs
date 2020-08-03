#![allow(dead_code)]

type Currency = String;

const RUB: Currency = "RUB".to_string();
const USD: Currency = "USD".to_string();
const EUR: Currency = "EUR".to_string();
const TRY: Currency = "TRY".to_string();
const JPY: Currency = "JPY".to_string();
const CNY: Currency = "CNY".to_string();
const CHF: Currency = "CHF".to_string();
const GBP: Currency = "GBP".to_string();
const HKD: Currency = "HKD".to_string();

type OperationType = String;

const BUY: OperationType = "Buy".to_string();
const SELL: OperationType = "Sell".to_string();
const OperationTypeBrokerCommission: OperationType = "BrokerCommission".to_string();
const OperationTypeExchangeCommission: OperationType = "ExchangeCommission".to_string();
const OperationTypeServiceCommission: OperationType = "ServiceCommission".to_string();
const OperationTypeMarginCommission: OperationType = "MarginCommission".to_string();
const OperationTypeOtherCommission: OperationType = "OtherCommission".to_string();
const OperationTypePayIn: OperationType = "PayIn".to_string();
const OperationTypePayOut: OperationType = "PayOut".to_string();
const OperationTypeTax: OperationType = "Tax".to_string();
const OperationTypeTaxLucre: OperationType = "TaxLucre".to_string();
const OperationTypeTaxDividend: OperationType = "TaxDividend".to_string();
const OperationTypeTaxCoupon: OperationType = "TaxCoupon".to_string();
const OperationTypeTaxBack: OperationType = "TaxBack".to_string();
const OperationTypeRepayment: OperationType = "Repayment".to_string();
const OperationTypePartRepayment: OperationType = "PartRepayment".to_string();
const OperationTypeCoupon: OperationType = "Coupon".to_string();
const OperationTypeDividend: OperationType = "Dividend".to_string();
const OperationTypeSecurityIn: OperationType = "SecurityIn".to_string();
const OperationTypeSecurityOut: OperationType = "SecurityOut".to_string();
const OperationTypeBuyCard: OperationType = "BuyCard".to_string();

struct Instrument {
    FIGI: String,
    Ticker: String,
    ISIN: String,
    Name: String,
    MinPriceIncrement: f64,
    Lot: i64,
    Currency: Currency,
}

const RESTAPIURL: &str = "https://api-invest.tinkoff.ru/openapi";

struct RestClient {
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

    pub fn instrument_by_figi(figi: String) -> Instrument {}
}
