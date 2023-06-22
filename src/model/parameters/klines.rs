#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
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
    const SECOND: u64 = 1;
    const MINUTE: u64 = 60 * Self::SECOND;
    const HOUR: u64 = 60 * Self::MINUTE;
    const DAY: u64 = 24 * Self::HOUR;

    pub fn as_seconds(&self) -> u64 {
        match self {
            Klines::K1min => 1 * Self::MINUTE,
            Klines::K3min => 3 * Self::MINUTE,
            Klines::K5min => 5 * Self::MINUTE,
            Klines::K15min => 15 * Self::MINUTE,
            Klines::K30min => 30 * Self::MINUTE,
            Klines::K1hour => 1 * Self::HOUR,
            Klines::K2hour => 2 * Self::HOUR,
            Klines::K4hour => 4 * Self::HOUR,
            Klines::K6hour => 6 * Self::HOUR,
            Klines::K8hour => 8 * Self::HOUR,
            Klines::K12hour => 12 * Self::HOUR,
            Klines::K1day => 1 * Self::DAY,
            Klines::K1week => 7 * Self::DAY,
        }
    }

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

impl std::fmt::Display for Klines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
