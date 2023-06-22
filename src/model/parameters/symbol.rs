#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Symbol {
    BtcUsdt,
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::BtcUsdt => "BTC-USDT",
        }
    }
}
