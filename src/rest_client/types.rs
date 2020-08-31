// extern crate chrono;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

pub type Currency = String;

lazy_static! {
    pub static ref  RUB: Currency = String::from("RUB");
    pub static ref  USD: Currency = String::from("USD");
    pub static ref  EUR: Currency = String::from("EUR");
    pub static ref  TRY: Currency = String::from("TRY");
    pub static ref  JPY: Currency = String::from("JPY");
    pub static ref  CNY: Currency = String::from("CNY");
    pub static ref  CHF: Currency = String::from("CHF");
    pub static ref  GBP: Currency = String::from("GBP");
    pub static ref  HKD: Currency = String::from("HKD");
}

pub type OperationType = String;

lazy_static! {
    pub static ref  BUY: OperationType = String::from("Buy");
    pub static ref  SELL: OperationType = String::from("Sell");
    pub static ref  OperationTypeBrokerCommission: OperationType = String::from("BrokerCommission");
    pub static ref  OperationTypeExchangeCommission: OperationType = String::from("ExchangeCommission");
    pub static ref  OperationTypeServiceCommission: OperationType = String::from("ServiceCommission");
    pub static ref  OperationTypeMarginCommission: OperationType = String::from("MarginCommission");
    pub static ref  OperationTypeOtherCommission: OperationType = String::from("OtherCommission");
    pub static ref  OperationTypePayIn: OperationType = String::from("PayIn");
    pub static ref  OperationTypePayOut: OperationType = String::from("PayOut");
    pub static ref  OperationTypeTax: OperationType = String::from("Tax");
    pub static ref  OperationTypeTaxLucre: OperationType = String::from("TaxLucre");
    pub static ref  OperationTypeTaxDividend: OperationType = String::from("TaxDividend");
    pub static ref  OperationTypeTaxCoupon: OperationType = String::from("TaxCoupon");
    pub static ref  OperationTypeTaxBack: OperationType = String::from("TaxBack");
    pub static ref  OperationTypeRepayment: OperationType = String::from("Repayment");
    pub static ref  OperationTypePartRepayment: OperationType = String::from("PartRepayment");
    pub static ref  OperationTypeCoupon: OperationType = String::from("Coupon");
    pub static ref  OperationTypeDividend: OperationType = String::from("Dividend");
    pub static ref  OperationTypeSecurityIn: OperationType = String::from("SecurityIn");
    pub static ref  OperationTypeSecurityOut: OperationType = String::from("SecurityOut");
    pub static ref  OperationTypeBuyCard: OperationType = String::from("BuyCard");
}

pub type OrderStatus = String;

lazy_static! {
    pub static ref  OrderStatusNew: OrderStatus = String::from("New");
    pub static ref  OrderStatusPartiallyFill: OrderStatus = String::from("PartiallyFill");
    pub static ref  OrderStatusFill: OrderStatus = String::from("Fill");
    pub static ref  OrderStatusCancelled: OrderStatus = String::from("Cancelled");
    pub static ref  OrderStatusReplaced: OrderStatus = String::from("Replaced");
    pub static ref  OrderStatusPendingCancel: OrderStatus = String::from("PendingCancel");
    pub static ref  OrderStatusRejected: OrderStatus = String::from("Rejected");
    pub static ref  OrderStatusPendingReplace: OrderStatus = String::from("PendingReplace");
    pub static ref  OrderStatusPendingNew: OrderStatus = String::from("PendingNew");
}

pub type OperationStatus = String;

lazy_static! {
    pub static ref  OperationStatusDone: OperationStatus = String::from("Done");
    pub static ref  OperationStatusDecline: OperationStatus = String::from("Decline");
    pub static ref  OperationStatusProgress: OperationStatus = String::from("Progress");
}

pub type InstrumentType = String;

lazy_static! {
    pub static ref  InstrumentTypeStock: InstrumentType = String::from("Stock");
    pub static ref  InstrumentTypeCurrency: InstrumentType = String::from("Currency");
    pub static ref  InstrumentTypeBond: InstrumentType = String::from("Bond");
    pub static ref  InstrumentTypeEtf: InstrumentType = String::from("Etf");
}

pub type OrderType = String;

lazy_static! {
    pub static ref  OrderTypeLimit: OrderType = String::from("Limit");
    pub static ref  OrderTypeMarket: OrderType = String::from("Market");
}
pub struct PlacedOrder {
    id: String,
    operation: OperationType,
    status: OrderStatus,
    RejectReason: String,
    RequestedLots: i64,
    ExecutedLots: i64,
    Commission: MoneyAmount,
    Message: String,
}

pub struct Order {
    ID: String,
    FIGI: String,
    Operation: OperationType,
    Status: OrderStatus,
    RequestedLots: i64,
    ExecutedLots: i64,
    Type: OrderType,
    Price: f64,
}

pub struct Portfolio {
    Positions: Vec<PositionBalance>,
    // array or slice
    Currencies: Vec<CurrencyBalance>, // array or slice
}

pub struct CurrencyBalance {
    Currency: Currency,
    Balance: f64,
    Blocked: f64,
}

pub struct PositionBalance {
    FIGI: String,
    Ticker: String,
    ISIN: String,
    InstrumentType: InstrumentType,
    Balance: f64,
    Blocked: f64,
    Lots: i64,
    ExpectedYield: MoneyAmount,
    AveragePositionPrice: MoneyAmount,
    AveragePositionPriceNoNkd: MoneyAmount,
    Name: String,
}

pub struct MoneyAmount {
    Currency: Currency,
    Value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    FIGI: String,
    Ticker: String,
    ISIN: String,
    Name: String,
    MinPriceIncrement: f64,
    Lot: i64,
    Currency: Currency,
    Type: InstrumentType,
}

pub struct Operation {
    ID: String,
    Status: OperationStatus,
    Trades: Vec<Trade>,
    // array
    Commission: MoneyAmount,
    Currency: Currency,
    Payment: f64,
    Price: f64,
    Quantity: i64,
    FIGI: String,
    InstrumentType: InstrumentType,
    IsMarginCall: bool,
    DateTime: DateTime<Utc>,
    // https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
    OperationType: OperationType,
}

pub struct Trade {
    ID: String,
    DateTime: DateTime<Utc>,
    // https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
    Price: f64,
    Quantity: i64,
}

pub struct RestPriceQuantity {
    Price: f64,
    Quantity: f64,
}

pub type TradingStatus = String;

pub struct RestOrderBook {
    FIGI: String,
    Depth: i64,
    Bids: Vec<RestPriceQuantity>,
    // array
    Asks: Vec<RestPriceQuantity>,
    // array
    TradeStatus: TradingStatus,
    MinPriceIncrement: f64,
    LastPrice: f64,
    ClosePrice: f64,
    LimitUp: f64,
    LimitDown: f64,
    FaceValue: f64,
}

pub type AccountType = String;

lazy_static! {
    pub static ref  AccountTinkoff: AccountType = String::from("Tinkoff");
    pub static ref  AccountTinkoffIIS: AccountType = String::from("TinkoffIis");
}
pub struct Account {
    Type: AccountType,
    ID: String,
}
lazy_static! {
    pub static ref  DefaultAccount: String = String::from(""); // Номер счета (по умолчанию - Тинькофф)
}