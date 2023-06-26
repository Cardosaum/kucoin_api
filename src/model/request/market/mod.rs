#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct CandleRequest {
    pub klines: crate::model::parameters::klines::Klines,
    pub symbol: crate::model::parameters::symbol::Symbol,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl CandleRequest {
    pub fn new(
        klines: crate::model::parameters::klines::Klines,
        symbol: crate::model::parameters::symbol::Symbol,
        start_at: Option<chrono::DateTime<chrono::Utc>>,
        end_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        CandleRequest { klines, symbol, start_at, end_at }
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_endpoint(&self) -> String {
        let mut endpoint = String::from("/api/v1/market/candles?");
        endpoint.push_str(&format!("type={klines}", klines = self.klines.as_str()));
        endpoint.push_str(&format!("&symbol={}", self.symbol.as_str()));
        if let Some(t) = self.start_at {
            endpoint.push_str(&format!("&startAt={}", t.timestamp()));
        }
        if let Some(t) = self.end_at {
            endpoint.push_str(&format!("&endAt={}", t.timestamp()));
        }
        tracing::trace!(endpoint);
        endpoint
    }
}
