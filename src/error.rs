#[derive(thiserror::Error, Debug)]
pub enum APIError {
    #[error("Serde (de)serialization error")]
    Serde(#[from] serde_json::Error),
    #[error("Websocket error")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("REST Call error")]
    HTTP(#[from] reqwest::Error),
    #[error("Generic error")]
    Generic(#[from] anyhow::Error),
}