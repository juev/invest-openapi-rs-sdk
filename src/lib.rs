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

#[derive(Debug, Serialize, Deserialize)]
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
pub struct Orders {
    orders: Vec<Order>
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

pub struct Accounts {
    accounts: Vec<Account>
}

lazy_static! {
    pub static ref  DEFAULT_ACCOUNT: String = String::from(""); // Номер счета (по умолчанию - Тинькофф)
}

// streaming_domain.go

pub static MAX_ORDERBOOK_DEPTH: i64 = 20;

pub type CandleInterval = String;

lazy_static! {
pub static ref CandleInterval1Min:   CandleInterval = "1min";
pub static ref CandleInterval2Min:   CandleInterval = "2min";
pub static ref CandleInterval3Min:   CandleInterval = "3min";
pub static ref CandleInterval5Min:   CandleInterval = "5min";
pub static ref CandleInterval10Min:  CandleInterval = "10min";
pub static ref CandleInterval15Min:  CandleInterval = "15min";
pub static ref CandleInterval30Min:  CandleInterval = "30min";
pub static ref CandleInterval1Hour:  CandleInterval = "hour";
pub static ref CandleInterval2Hour:  CandleInterval = "2hour";
pub static ref CandleInterval4Hour:  CandleInterval = "4hour";
pub static ref CandleInterval1Day:   CandleInterval = "day";
pub static ref CandleInterval1Week:  CandleInterval = "week";
pub static ref CandleInterval1Month: CandleInterval = "month";
}

lazy_static! {
pub static ref BreakInTrading:               TradingStatus = "break_in_trading";
pub static ref NormalTrading:                TradingStatus = "normal_trading";
pub static ref NotAvailableForTrading:       TradingStatus = "not_available_for_trading";
pub static ref ClosingAuction:               TradingStatus = "closing_auction";
pub static ref ClosingPeriod:                TradingStatus = "closing_period";
pub static ref DarkPoolAuction:              TradingStatus = "dark_pool_auction";
pub static ref DiscreteAuction:              TradingStatus = "discrete_auction";
pub static ref OpeningPeriod:                TradingStatus = "opening_period";
pub static ref OpeningAuctionPeriod:         TradingStatus = "opening_auction_period";
pub static ref TradingAtClosingAuctionPrice: TradingStatus = "trading_at_closing_auction_price";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "event")]
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullEvent {
    #[serde(rename = "event")]
    name: String,
    time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleEvent {
    full_event: FullEvent,
    #[serde(rename = "payload")]
    candle: Candle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    figi: String,
    interval: CandleInterval,
    #[serde(rename = "o")]
    open_price: f64,
    #[serde(rename = "c")]
    close_price: f64,
    #[serde(rename = "h")]
    high_price: f64,
    #[serde(rename = "l")]
    low_price: f64,
    #[serde(rename = "v")]
    volume: f64,
    #[serde(rename = "time")]
    ts: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candles {
    candles: Vec<Candle>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookEvent {
    full_event: FullEvent,
    #[serde(rename = "payload")]
    order_book: OrderBook,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    figi: String,
    depth: i64,
    bids: Vec<PriceQuantity>,
    asks: Vec<PriceQuantity>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceQuantity {
    price: f64,
    quantity: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentInfoEvent {
    full_event: FullEvent,
    #[serde(rename = "payload")]
    info: InstrumentInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentInfo {
    figi: String,
    trade_status: TradingStatus,
    min_price_increment: f64,
    lot: f64,
    #[serde(skip_deserializing)]
    accrued_interest: f64,
    #[serde(skip_deserializing)]
    limit_up: f64,
    #[serde(skip_deserializing)]
    limit_down: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEvent {
    full_event: FullEvent,
    #[serde(rename = "payload")]
    error: Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    request_id: String,
    #[serde(skip_deserializing)]
    error: String,
}
