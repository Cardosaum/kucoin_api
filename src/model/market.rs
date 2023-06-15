use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde_this_or_that::as_f64;
use serde_with::{serde_as, TimestampSeconds};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid order book type")]
    InvalidOrderBookType,
    #[error("Invalid klines")]
    InvalidKlines,
    #[error("Invalid chain")]
    InvalidChain,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    #[serde_as(as = "TimestampSeconds<String>")]
    time: DateTime<Utc>,
    #[serde(deserialize_with = "as_f64")]
    open: f64,
    #[serde(deserialize_with = "as_f64")]
    close: f64,
    #[serde(deserialize_with = "as_f64")]
    high: f64,
    #[serde(deserialize_with = "as_f64")]
    low: f64,
    #[serde(deserialize_with = "as_f64")]
    volume: f64,
    #[serde(deserialize_with = "as_f64")]
    amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolList {
    pub symbol: String,
    pub name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub quote_max_size: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub price_increment: String,
    pub fee_currency: String,
    pub enable_trading: bool,
    pub is_margin_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub sequence: String,
    pub best_ask: String,
    pub size: String,
    pub price: String,
    pub best_bid_size: String,
    pub best_bid: String,
    pub best_ask_size: String,
    pub time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllTickers {
    pub time: i64,
    pub ticker: Vec<Tick>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    pub symbol: String,
    pub symbol_name: String,
    pub buy: String,
    pub sell: String,
    pub change_rate: Option<String>,
    pub change_price: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub vol: String,
    pub vol_value: String,
    pub last: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStats {
    pub symbol: String,
    pub buy: String,
    pub sell: String,
    pub change_rate: Option<String>,
    pub change_price: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub vol: String,
    pub vol_value: String,
    pub last: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub sequence: String,
    pub time: i64,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicOrderBook {
    pub sequence: i64,
    pub time: i64,
    pub bids: Vec<(String, String, String, i64)>,
    pub asks: Vec<(String, String, String, i64)>,
}

pub enum OrderBookType {
    L20,
    L100,
    Full,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistories {
    pub sequence: String,
    pub price: String,
    pub size: String,
    pub side: String,
    pub time: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Klines {
    K1min,
    K3min,
    K5min,
    K15min,
    K30min,
    K1hour,
    K2hour,
    K4hour,
    K6hour,
    K8hour,
    K12hour,
    K1day,
    K1week,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    currency: String,
    name: String,
    full_name: String,
    precision: i32,
    withdrawal_min_size: String,
    withdrawal_min_fee: String,
    is_withdrawal_enabled: Option<bool>,
    is_deposit_enabled: bool,
    is_margin_enabled: bool,
    is_debit_enabled: bool,
}

pub enum Chain {
    Native,
    Segwit,
    OMNI,
    ERC20,
    TRC20,
}

impl Chain {
    pub const VARIANTS: [&'static str; 5] = ["Native", "Segwit", "OMNI", "ERC20", "TRC20"];

    pub fn as_str(&self) -> &str {
        match self {
            Chain::Native => "Native",
            Chain::Segwit => "Segwit",
            Chain::OMNI => "OMNI",
            Chain::ERC20 => "ERC20",
            Chain::TRC20 => "TRC20",
        }
    }
}

impl FromStr for Chain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Native" => Ok(Chain::Native),
            "Segwit" => Ok(Chain::Segwit),
            "OMNI" => Ok(Chain::OMNI),
            "ERC20" => Ok(Chain::ERC20),
            "TRC20" => Ok(Chain::TRC20),
            _ => Err(Error::InvalidChain),
        }
    }
}

pub enum Fiat {
    USD,
    EUR,
    CAD,
    CNY,
    AUD,
    KRW,
    JPY,
    GBP,
    INR,
    IDR,
    RUB,
    BRL,
    TRY,
    PLN,
    PHP,
    ZAR,
    THB,
    CHF,
    MYR,
    MXR,
    HRK,
    ARS,
    KZT,
    IRR,
    VND,
    ILS,
    BDT,
    HKD,
    TWD,
    COP,
    DKK,
    BGN,
    NOK,
    DZD,
    RON,
    SGD,
    NGN,
    CZK,
    PKR,
    SEK,
    NZD,
    UAH,
}

impl Klines {
    pub const VARIANTS: [&'static str; 13] = [
        "1min", "3min", "5min", "15min", "30min", "1hour", "2hour", "4hour", "6hour", "8hour",
        "12hour", "1day", "1week",
    ];

    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * 60;
    const DAY: u64 = 60 * 60 * 24;
    const WEEK: u64 = 60 * 60 * 24 * 7;
    pub fn as_seconds(&self) -> u64 {
        match self {
            Klines::K1min => Self::MINUTE,
            Klines::K3min => Self::MINUTE * 3,
            Klines::K5min => Self::MINUTE * 5,
            Klines::K15min => Self::MINUTE * 15,
            Klines::K30min => Self::MINUTE * 30,
            Klines::K1hour => Self::HOUR,
            Klines::K2hour => Self::HOUR * 2,
            Klines::K4hour => Self::HOUR * 4,
            Klines::K6hour => Self::HOUR * 6,
            Klines::K8hour => Self::HOUR * 8,
            Klines::K12hour => Self::HOUR * 12,
            Klines::K1day => Self::DAY,
            Klines::K1week => Self::WEEK,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Klines::K1min => "1min",
            Klines::K3min => "3min",
            Klines::K5min => "5min",
            Klines::K15min => "15min",
            Klines::K30min => "30min",
            Klines::K1hour => "1hour",
            Klines::K2hour => "2hour",
            Klines::K4hour => "4hour",
            Klines::K6hour => "6hour",
            Klines::K8hour => "8hour",
            Klines::K12hour => "12hour",
            Klines::K1day => "1day",
            Klines::K1week => "1week",
        }
    }
}

impl std::fmt::Display for SymbolList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for AllTickers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for DailyStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for OrderBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for AtomicOrderBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for TradeHistories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl std::fmt::Display for Klines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl OrderBookType {
    pub const VARIANTS: [&'static str; 3] = ["L20", "L100", "Full"];

    pub fn as_str(&self) -> &str {
        match self {
            OrderBookType::L20 => "L20",
            OrderBookType::L100 => "L100",
            OrderBookType::Full => "Full",
        }
    }
}

impl FromStr for OrderBookType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L20" => Ok(OrderBookType::L20),
            "L100" => Ok(OrderBookType::L100),
            "Full" => Ok(OrderBookType::Full),
            _ => Err(Error::InvalidOrderBookType),
        }
    }
}

impl FromStr for Klines {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().parse()
    }
}

impl FromStr for &Klines {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1min" => Ok(&Klines::K1min),
            "3min" => Ok(&Klines::K3min),
            "5min" => Ok(&Klines::K5min),
            "15min" => Ok(&Klines::K15min),
            "30min" => Ok(&Klines::K30min),
            "1hour" => Ok(&Klines::K1hour),
            "2hour" => Ok(&Klines::K2hour),
            "4hour" => Ok(&Klines::K4hour),
            "6hour" => Ok(&Klines::K6hour),
            "8hour" => Ok(&Klines::K8hour),
            "12hour" => Ok(&Klines::K12hour),
            "1day" => Ok(&Klines::K1day),
            "1week" => Ok(&Klines::K1week),
            _ => Err(Error::InvalidOrderBookType),
        }
    }
}
