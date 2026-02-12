/// Configuration for connecting to the Alpaca API.
#[derive(Debug, Clone)]
pub struct AlpacaConfig {
    pub api_key_id: String,
    pub api_secret_key: String,
    pub trading_base_url: String,
    pub market_data_base_url: String,
}

impl AlpacaConfig {
    /// Create config from environment variables.
    ///
    /// Required: `APCA_API_KEY_ID`, `APCA_API_SECRET_KEY`
    /// Optional: `APCA_TRADING_BASE_URL`, `APCA_MARKET_DATA_BASE_URL`
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            api_key_id: std::env::var("APCA_API_KEY_ID")?,
            api_secret_key: std::env::var("APCA_API_SECRET_KEY")?,
            trading_base_url: std::env::var("APCA_TRADING_BASE_URL")
                .unwrap_or_else(|_| "https://paper-api.alpaca.markets".into()),
            market_data_base_url: std::env::var("APCA_MARKET_DATA_BASE_URL")
                .unwrap_or_else(|_| "https://data.alpaca.markets".into()),
        })
    }

    /// Create config for paper trading.
    pub fn paper(api_key_id: String, api_secret_key: String) -> Self {
        Self {
            api_key_id,
            api_secret_key,
            trading_base_url: "https://paper-api.alpaca.markets".into(),
            market_data_base_url: "https://data.alpaca.markets".into(),
        }
    }
}
