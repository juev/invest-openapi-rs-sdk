#![allow(dead_code)]
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub mod rest_client;

pub type Currency = String;

lazy_static! {
    pub static ref RUB: Currency = String::from("RUB");
    pub static ref USD: Currency = String::from("USD");
    pub static ref EUR: Currency = String::from("EUR");
    pub static ref TRY: Currency = String::from("TRY");
    pub static ref JPY: Currency = String::from("JPY");
    pub static ref CNY: Currency = String::from("CNY");
    pub static ref CHF: Currency = String::from("CHF");
    pub static ref GBP: Currency = String::from("GBP");
    pub static ref HKD: Currency = String::from("HKD");
}

pub type OperationType = String;

lazy_static! {
    pub static ref OPERATION_TYPE_BUY: OperationType = String::from("Buy");
    pub static ref OPERATION_TYPE_SELL: OperationType = String::from("Sell");
    pub static ref OPERATION_TYPE_BROKER_COMMISSION: OperationType =
        String::from("BrokerCommission");
    pub static ref OPERATION_TYPE_EXCHANGE_COMMISSION: OperationType =
        String::from("ExchangeCommission");
    pub static ref OPERATION_TYPE_SERVICE_COMMISSION: OperationType =
        String::from("ServiceCommission");
    pub static ref OPERATION_TYPE_MARGIN_COMMISSION: OperationType =
        String::from("MarginCommission");
    pub static ref OPERATION_TYPE_OTHER_COMMISSION: OperationType = String::from("OtherCommission");
    pub static ref OPERATION_TYPE_PAY_IN: OperationType = String::from("PayIn");
    pub static ref OPERATION_TYPE_PAY_OUT: OperationType = String::from("PayOut");
    pub static ref OPERATION_TYPE_TAX: OperationType = String::from("Tax");
    pub static ref OPERATION_TYPE_TAX_LUCRE: OperationType = String::from("TaxLucre");
    pub static ref OPERATION_TYPE_TAX_DIVIDEND: OperationType = String::from("TaxDividend");
    pub static ref OPERATION_TYPE_TAX_COUPON: OperationType = String::from("TaxCoupon");
    pub static ref OPERATION_TYPE_TAX_BACK: OperationType = String::from("TaxBack");
    pub static ref OPERATION_TYPE_REPAYMENT: OperationType = String::from("Repayment");
    pub static ref OPERATION_TYPE_PART_REPAYMENT: OperationType = String::from("PartRepayment");
    pub static ref OPERATION_TYPE_COUPON: OperationType = String::from("Coupon");
    pub static ref OPERATION_TYPE_DIVIDEND: OperationType = String::from("Dividend");
    pub static ref OPERATION_TYPE_SECURITY_IN: OperationType = String::from("SecurityIn");
    pub static ref OPERATION_TYPE_SECURITY_OUT: OperationType = String::from("SecurityOut");
    pub static ref OPERATION_TYPE_BUY_CARD: OperationType = String::from("BuyCard");
}

pub type OrderStatus = String;

lazy_static! {
    pub static ref ORDER_STATUS_NEW: OrderStatus = String::from("New");
    pub static ref ORDER_STATUS_PARTIALLY_FILL: OrderStatus = String::from("PartiallyFill");
    pub static ref ORDER_STATUS_FILL: OrderStatus = String::from("Fill");
    pub static ref ORDER_STATUS_CANCELLED: OrderStatus = String::from("Cancelled");
    pub static ref ORDER_STATUS_REPLACED: OrderStatus = String::from("Replaced");
    pub static ref ORDER_STATUS_PENDING_CANCEL: OrderStatus = String::from("PendingCancel");
    pub static ref ORDER_STATUS_REJECTED: OrderStatus = String::from("Rejected");
    pub static ref ORDER_STATUS_PENDING_REPLACE: OrderStatus = String::from("PendingReplace");
    pub static ref ORDER_STATUS_PENDING_NEW: OrderStatus = String::from("PendingNew");
}

pub type OperationStatus = String;

lazy_static! {
    pub static ref OPERATION_STATUS_DONE: OperationStatus = String::from("Done");
    pub static ref OPERATION_STATUS_DECLINE: OperationStatus = String::from("Decline");
    pub static ref OPERATION_STATUS_PROGRESS: OperationStatus = String::from("Progress");
}

pub type InstrumentType = String;

lazy_static! {
    pub static ref INSTRUMENT_TYPE_STOCK: InstrumentType = String::from("Stock");
    pub static ref INSTRUMENT_TYPE_CURRENCY: InstrumentType = String::from("currency");
    pub static ref INSTRUMENT_TYPE_BOND: InstrumentType = String::from("Bond");
    pub static ref INSTRUMENT_TYPE_ETF: InstrumentType = String::from("Etf");
}

pub type OrderType = String;

lazy_static! {
    pub static ref ORDER_TYPE_LIMIT: OrderType = String::from("Limit");
    pub static ref ORDER_TYPE_MARKET: OrderType = String::from("Market");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacedOrder {
    id: String,
    operation: OperationType,
    status: OrderStatus,
    reject_reason: String,
    requested_lots: i64,
    executed_lots: i64,
    commission: MoneyAmount,
    message: String,
}

pub struct Order {
    id: String,
    figi: String,
    operation: OperationType,
    status: OrderStatus,
    requested_lots: i64,
    executed_lots: i64,
    r#type: OrderType,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    pub positions: PositionBalances,
    pub currencies: CurrencyBalances,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyBalance {
    currency: Currency,
    balance: f64,
    #[serde(default)]
    blocked: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyBalances {
    currencies: Vec<CurrencyBalance>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionBalance {
    figi: String,
    ticker: String,
    isin: String,
    instrument_type: InstrumentType,
    balance: f64,
    blocked: f64,
    lots: i64,
    expected_yield: MoneyAmount,
    average_position_price: MoneyAmount,
    average_position_price_no_nkd: MoneyAmount,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionBalances {
    positions: Vec<PositionBalance>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAmount {
    currency: Currency,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    figi: String,
    ticker: String,
    #[serde(default)]
    isin: String,
    name: String,
    #[serde(default, rename = "minPriceIncrement")]
    min_price_increment: f64,
    lot: i64,
    currency: Currency,
    #[serde(rename = "type")]
    r#type: InstrumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruments {
    instruments: Vec<Instrument>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    id: String,
    status: OperationStatus,
    trades: Vec<Trade>,
    // array
    commission: MoneyAmount,
    currency: Currency,
    payment: f64,
    price: f64,
    quantity: i64,
    #[serde(rename = "quantityExecuted")]
    quantity_executed: i64,
    figi: String,
    #[serde(rename = "instrumentType")]
    instrument_type: InstrumentType,
    #[serde(rename = "isMarginCall")]
    is_margin_call: bool,
    #[serde(rename = "date")]
    date_time: DateTime<Utc>,
    // https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
    #[serde(rename = "operationType")]
    operation_type: OperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    operations: Vec<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    id: String,
    #[serde(rename = "date")]
    date_time: DateTime<Utc>,
    // https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
    price: f64,
    quantity: i64,
}

pub struct RestPriceQuantity {
    price: f64,
    quantity: f64,
}

pub type TradingStatus = String;

pub struct RestOrderBook {
    figi: String,
    depth: i64,
    bids: Vec<RestPriceQuantity>,
    // array
    asks: Vec<RestPriceQuantity>,
    // array
    trade_status: TradingStatus,
    min_price_increment: f64,
    last_price: f64,
    close_price: f64,
    limit_up: f64,
    limit_down: f64,
    face_value: f64,
}

pub type AccountType = String;

lazy_static! {
    pub static ref ACCOUNT_TINKOFF: AccountType = String::from("Tinkoff");
    pub static ref ACCOUNT_TINKOFF_IIS: AccountType = String::from("TinkoffIis");
}

pub struct Account {
    r#type: AccountType,
    id: String,
}
lazy_static! {
    pub static ref  DEFAULT_ACCOUNT: String = String::from(""); // Номер счета (по умолчанию - Тинькофф)
}
