#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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

impl Klines {
    pub fn as_str(&self) -> &'static str {
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
