use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlpacaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Alpaca API error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("JSON deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Rate limited, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    #[error("Configuration error: {0}")]
    Config(String),
}
