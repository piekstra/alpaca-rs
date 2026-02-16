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

    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

impl From<api_client_core::ApiClientError> for AlpacaError {
    fn from(err: api_client_core::ApiClientError) -> Self {
        match err {
            api_client_core::ApiClientError::Http(e) => AlpacaError::Http(e),
            api_client_core::ApiClientError::Api { status, body } => {
                AlpacaError::Api { status, body }
            }
            api_client_core::ApiClientError::Deserialize(e) => AlpacaError::Deserialize(e),
            api_client_core::ApiClientError::RateLimited { retry_after_secs } => {
                AlpacaError::RateLimited { retry_after_secs }
            }
            api_client_core::ApiClientError::Config(msg) => AlpacaError::Config(msg),
            api_client_core::ApiClientError::WebSocket(msg) => AlpacaError::WebSocket(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_display() {
        let err = AlpacaError::Api {
            status: 404,
            body: "not found".to_string(),
        };
        assert_eq!(err.to_string(), "Alpaca API error 404: not found");
    }

    #[test]
    fn rate_limited_display() {
        let err = AlpacaError::RateLimited {
            retry_after_secs: 5,
        };
        assert_eq!(err.to_string(), "Rate limited, retry after 5s");
    }

    #[test]
    fn config_error_display() {
        let err = AlpacaError::Config("missing key".to_string());
        assert_eq!(err.to_string(), "Configuration error: missing key");
    }

    #[test]
    fn websocket_error_display() {
        let err = AlpacaError::WebSocket("connection closed".to_string());
        assert_eq!(err.to_string(), "WebSocket error: connection closed");
    }

    #[test]
    fn from_core_api_error() {
        let core_err = api_client_core::ApiClientError::Api {
            status: 403,
            body: "forbidden".to_string(),
        };
        let alpaca_err: AlpacaError = core_err.into();
        match alpaca_err {
            AlpacaError::Api { status, body } => {
                assert_eq!(status, 403);
                assert_eq!(body, "forbidden");
            }
            _ => panic!("expected Api variant"),
        }
    }

    #[test]
    fn from_core_rate_limited() {
        let core_err = api_client_core::ApiClientError::RateLimited {
            retry_after_secs: 10,
        };
        let alpaca_err: AlpacaError = core_err.into();
        match alpaca_err {
            AlpacaError::RateLimited { retry_after_secs } => {
                assert_eq!(retry_after_secs, 10);
            }
            _ => panic!("expected RateLimited variant"),
        }
    }

    #[test]
    fn from_core_websocket_error() {
        let core_err = api_client_core::ApiClientError::WebSocket("timeout".to_string());
        let alpaca_err: AlpacaError = core_err.into();
        match alpaca_err {
            AlpacaError::WebSocket(msg) => assert_eq!(msg, "timeout"),
            _ => panic!("expected WebSocket variant"),
        }
    }
}
