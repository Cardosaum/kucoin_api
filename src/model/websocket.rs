use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid websocket topic")]
    InvalidWSTopic,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServers {
    pub instance_servers: Vec<InstanceServer>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    pub ping_interval: i32,
    pub endpoint: String,
    pub protocol: String,
    pub encrypt: bool,
    pub ping_timeout: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WSTopic {
    Ticker(Vec<String>),
    AllTicker,
    Snapshot(String),
    OrderBook(Vec<String>),
    OrderBookDepth5(Vec<String>),
    OrderBookDepth50(Vec<String>),
    Match(Vec<String>),
    FullMatch(Vec<String>),
    Level3Public(Vec<String>),
    Level3Private(Vec<String>),
    IndexPrice(Vec<String>),
    MarketPrice(Vec<String>),
    OrderBookChange(Vec<String>),
    StopOrder(Vec<String>),
    Balances,
    DebtRatio,
    PositionChange,
    MarginTradeOrder(String),
    TradeOrders,
}

impl WSTopic {
    pub fn as_str(&self) -> &str {
        match self {
            WSTopic::Ticker(_) => "ticker",
            WSTopic::AllTicker => "allTicker",
            WSTopic::Snapshot(_) => "snapshot",
            WSTopic::OrderBook(_) => "orderBook",
            WSTopic::OrderBookDepth5(_) => "orderBookDepth5",
            WSTopic::OrderBookDepth50(_) => "orderBookDepth50",
            WSTopic::Match(_) => "match",
            WSTopic::FullMatch(_) => "fullMatch",
            WSTopic::Level3Public(_) => "level3public",
            WSTopic::Level3Private(_) => "level3private",
            WSTopic::IndexPrice(_) => "indexPrice",
            WSTopic::MarketPrice(_) => "marketPrice",
            WSTopic::OrderBookChange(_) => "orderBookChange",
            WSTopic::StopOrder(_) => "stopOrder",
            WSTopic::Balances => "balances",
            WSTopic::DebtRatio => "debtRatio",
            WSTopic::PositionChange => "positionChange",
            WSTopic::MarginTradeOrder(_) => "marginTradeOrder",
            WSTopic::TradeOrders => "tradeOrders",
        }
    }
}

impl FromStr for WSTopic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ticker" => Ok(WSTopic::Ticker(vec![])),
            "allTicker" => Ok(WSTopic::AllTicker),
            "snapshot" => Ok(WSTopic::Snapshot("".to_string())),
            "orderBook" => Ok(WSTopic::OrderBook(vec![])),
            "orderBookDepth5" => Ok(WSTopic::OrderBookDepth5(vec![])),
            "orderBookDepth50" => Ok(WSTopic::OrderBookDepth50(vec![])),
            "match" => Ok(WSTopic::Match(vec![])),
            "fullMatch" => Ok(WSTopic::FullMatch(vec![])),
            "level3public" => Ok(WSTopic::Level3Public(vec![])),
            "level3private" => Ok(WSTopic::Level3Private(vec![])),
            "indexPrice" => Ok(WSTopic::IndexPrice(vec![])),
            "marketPrice" => Ok(WSTopic::MarketPrice(vec![])),
            "orderBookChange" => Ok(WSTopic::OrderBookChange(vec![])),
            "stopOrder" => Ok(WSTopic::StopOrder(vec![])),
            "balances" => Ok(WSTopic::Balances),
            "debtRatio" => Ok(WSTopic::DebtRatio),
            "positionChange" => Ok(WSTopic::PositionChange),
            "marginTradeOrder" => Ok(WSTopic::MarginTradeOrder("".to_string())),
            "tradeOrders" => Ok(WSTopic::TradeOrders),
            _ => Err(Error::InvalidWSTopic),
        }
    }
}
pub enum WSType {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum KucoinWebsocketMsg {
    WelcomeMsg(DefaultMsg),
    SubscribeMsg(Subscribe),
    PingMsg(DefaultMsg),
    PongMsg(DefaultMsg),
    Ping,
    Pong,
    Binary(Vec<u8>),
    TickerMsg(WSResp<SymbolTicker>),
    AllTickerMsg(WSResp<SymbolTicker>),
    SnapshotMsg(WSResp<Snapshot>),
    OrderBookMsg(WSResp<Level2>),
    MatchMsg(WSResp<Match>),
    Level3ReceivedMsg(WSResp<Level3Received>),
    Level3OpenMsg(WSResp<Level3Open>),
    Level3MatchMsg(WSResp<Level3Match>),
    Level3DoneMsg(WSResp<Level3Done>),
    Level3ChangeMsg(WSResp<Level3Change>),
    OrderBookDepthMsg(WSResp<Level2Depth>),
    FullMatchReceivedMsg(WSResp<FullMatchReceived>),
    FullMatchOpenMsg(WSResp<FullMatchOpen>),
    FullMatchDoneMsg(WSResp<FullMatchDone>),
    FullMatchMatchMsg(WSResp<FullMatchMatch>),
    FullMatchChangeMsg(WSResp<FullMatchChange>),
    IndexPriceMsg(WSResp<IndexPrice>),
    MarketPriceMsg(WSResp<MarketPrice>),
    OrderBookChangeMsg(WSResp<BookChange>),
    StopOrderMsg(WSResp<StopOrder>),
    BalancesMsg(WSResp<Balances>),
    DebtRatioMsg(WSResp<DebtRatio>),
    PositionChangeMsg(WSResp<PositionChange>),
    MarginTradeOpenMsg(WSResp<MarginTradeOpen>),
    MarginTradeUpdateMsg(WSResp<MarginTradeUpdate>),
    MarginTradeDoneMsg(WSResp<MarginTradeDone>),
    TradeOpenMsg(WSResp<TradeOpen>),
    TradeMatchMsg(WSResp<TradeMatch>),
    TradeFilledMsg(WSResp<TradeFilled>),
    TradeCanceledMsg(WSResp<TradeCanceled>),
    TradeUpdateMsg(WSResp<TradeUpdate>),
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResp<T> {
    pub r#type: String,
    pub topic: String,
    pub subject: String,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultMsg {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    pub id: String,
    pub r#type: String,
    pub topic: String,
    pub private_channel: bool,
    pub response: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolTicker {
    pub sequence: String,
    pub best_ask: String,
    pub size: String,
    pub best_bid_size: String,
    pub price: String,
    pub best_ask_size: String,
    pub best_bid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub sequence: i64,
    pub data: SnapshotData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotData {
    pub trading: bool,
    pub symbol: String,
    pub buy: f32,
    pub sell: f32,
    pub sort: i32,
    pub vol_value: f32,
    pub base_currency: String,
    pub market: String,
    pub quote_currency: String,
    pub symbol_code: String,
    pub datetime: i64,
    pub high: Option<f32>,
    pub vol: f32,
    pub low: Option<f32>,
    pub change_price: Option<f32>,
    pub change_rate: f32,
    pub last_traded_price: f32,
    pub board: i32,
    pub mark: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2 {
    pub sequence_start: i64,
    pub sequence_end: i64,
    pub symbol: String,
    pub changes: Level2Changes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2Depth {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2Changes {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Received {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: Option<String>,
    pub time: String,
    pub client_oid: Option<String>,
    pub r#type: String,
    pub order_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Open {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub order_id: String,
    pub price: String,
    pub time: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Done {
    pub sequence: String,
    pub symbol: String,
    pub reason: String,
    pub side: String,
    pub order_id: String,
    pub time: String,
    pub r#type: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Change {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: String,
    pub new_size: String,
    pub time: String,
    pub r#type: String,
    pub old_size: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchReceived {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub client_oid: Option<String>,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchOpen {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub order_time: i64,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchDone {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub reason: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchMatch {
    pub sequence: i64,
    pub symbol: String,
    pub side: String,
    pub price: String,
    pub remain_size: String,
    pub taker_order_id: String,
    pub maker_order_id: String,
    pub trade_id: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchChange {
    pub sequence: i64,
    pub symbol: String,
    pub size: String,
    pub order_id: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookChange {
    pub sequence: i32,
    pub currency: String,
    pub daily_int_rate: f32,
    pub annual_int_rate: f32,
    pub term: i32,
    pub size: f32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrder {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub stop_entry: String,
    pub funds: String,
    pub time: String,
    pub r#type: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub total: String,
    pub available: String,
    pub available_change: String,
    pub currency: String,
    pub hold: String,
    pub hold_change: String,
    pub relation_event: String,
    pub relation_event_id: String,
    pub time: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebtRatio {
    pub debt_ratio: f32,
    pub total_debt: String,
    pub debt_list: HashMap<String, String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionChange {
    pub r#type: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeOpen {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeUpdate {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub lent_size: f32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeDone {
    pub currency: String,
    pub order_id: String,
    pub reason: String,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOpen {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeMatch {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub liquidity: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    #[serde(default)]
    pub price: String,
    pub match_price: String,
    pub match_size: String,
    pub trade_id: String,
    #[serde(default)]
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeFilled {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeCanceled {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeUpdate {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub old_size: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

impl Display for KucoinWebsocketMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl<T: Serialize> Display for WSResp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}
