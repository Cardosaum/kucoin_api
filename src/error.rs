#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde (de)serialization error")]
    Serde(#[from] serde_json::Error),
    #[error("Websocket error")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("REST Call error")]
    HTTP(#[from] reqwest::Error),
    #[error("Failed to join async task error")]
    Join(#[from] tokio::task::JoinError),
    #[error("Generic error")]
    Generic(#[from] anyhow::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
