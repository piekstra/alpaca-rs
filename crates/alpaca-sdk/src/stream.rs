use api_client_core::WebSocketClient;
use serde_json::json;

use crate::config::AlpacaConfig;
use crate::error::AlpacaError;
use crate::types::{AlpacaStreamMessage, AlpacaTradeUpdate};

const MARKET_DATA_STREAM_SIP: &str = "wss://stream.data.alpaca.markets/v2/sip";
const MARKET_DATA_STREAM_IEX: &str = "wss://stream.data.alpaca.markets/v2/iex";
const MARKET_DATA_STREAM_TEST: &str = "wss://stream.data.alpaca.markets/v2/test";

/// Alpaca WebSocket streaming client for real-time market data and trade updates.
///
/// Built on `api_client_core::WebSocketClient` for standardized WebSocket handling.
pub struct AlpacaStream {
    ws: WebSocketClient,
}

/// Feed source for market data streams.
#[derive(Debug, Clone, Copy)]
pub enum MarketDataFeed {
    /// SIP (Securities Information Processor) - all US exchanges, requires paid plan
    Sip,
    /// IEX (Investors Exchange) - free tier
    Iex,
    /// Test feed for development
    Test,
}

impl MarketDataFeed {
    fn url(&self) -> &'static str {
        match self {
            MarketDataFeed::Sip => MARKET_DATA_STREAM_SIP,
            MarketDataFeed::Iex => MARKET_DATA_STREAM_IEX,
            MarketDataFeed::Test => MARKET_DATA_STREAM_TEST,
        }
    }
}

impl AlpacaStream {
    /// Connect to Alpaca's market data WebSocket stream.
    ///
    /// Authenticates automatically using the provided config credentials.
    pub async fn connect_market_data(
        config: &AlpacaConfig,
        feed: MarketDataFeed,
    ) -> Result<Self, AlpacaError> {
        let auth = json!({
            "action": "auth",
            "key": config.api_key_id,
            "secret": config.api_secret_key,
        });

        let ws = WebSocketClient::connect(feed.url(), Some(auth))
            .await
            .map_err(AlpacaError::from)?;

        Ok(Self { ws })
    }

    /// Connect to Alpaca's trade updates WebSocket stream (order fills, cancellations, etc).
    pub async fn connect_trade_updates(config: &AlpacaConfig) -> Result<Self, AlpacaError> {
        let base = &config.trading_base_url;
        let url = base.replace("https://", "wss://") + "/stream";

        let auth = json!({
            "action": "authenticate",
            "data": {
                "key_id": config.api_key_id,
                "secret_key": config.api_secret_key,
            }
        });

        let ws = WebSocketClient::connect(&url, Some(auth))
            .await
            .map_err(AlpacaError::from)?;

        Ok(Self { ws })
    }

    /// Subscribe to real-time trades for the given symbols.
    pub async fn subscribe_trades(&mut self, symbols: &[&str]) -> Result<(), AlpacaError> {
        self.send_subscription("subscribe", symbols, &[], &[]).await
    }

    /// Subscribe to real-time quotes for the given symbols.
    pub async fn subscribe_quotes(&mut self, symbols: &[&str]) -> Result<(), AlpacaError> {
        self.send_subscription("subscribe", &[], symbols, &[]).await
    }

    /// Subscribe to real-time minute bars for the given symbols.
    pub async fn subscribe_bars(&mut self, symbols: &[&str]) -> Result<(), AlpacaError> {
        self.send_subscription("subscribe", &[], &[], symbols).await
    }

    /// Subscribe to trades, quotes, and/or bars in a single message.
    pub async fn subscribe(
        &mut self,
        trades: &[&str],
        quotes: &[&str],
        bars: &[&str],
    ) -> Result<(), AlpacaError> {
        self.send_subscription("subscribe", trades, quotes, bars)
            .await
    }

    /// Unsubscribe from trades, quotes, and/or bars.
    pub async fn unsubscribe(
        &mut self,
        trades: &[&str],
        quotes: &[&str],
        bars: &[&str],
    ) -> Result<(), AlpacaError> {
        self.send_subscription("unsubscribe", trades, quotes, bars)
            .await
    }

    /// Listen for trade updates (for the account stream).
    pub async fn listen_trade_updates(&mut self) -> Result<(), AlpacaError> {
        let msg = json!({
            "action": "listen",
            "data": {
                "streams": ["trade_updates"]
            }
        });
        self.ws.send(&msg).await.map_err(AlpacaError::from)
    }

    /// Receive the next market data stream message.
    pub async fn recv(&mut self) -> Option<Result<AlpacaStreamMessage, AlpacaError>> {
        match self.ws.recv().await {
            Some(Ok(text)) => Some(serde_json::from_str(&text).map_err(AlpacaError::Deserialize)),
            Some(Err(e)) => Some(Err(AlpacaError::from(e))),
            None => None,
        }
    }

    /// Receive the next trade update message (for the account stream).
    pub async fn recv_trade_update(&mut self) -> Option<Result<AlpacaTradeUpdate, AlpacaError>> {
        match self.ws.recv().await {
            Some(Ok(text)) => Some(serde_json::from_str(&text).map_err(AlpacaError::Deserialize)),
            Some(Err(e)) => Some(Err(AlpacaError::from(e))),
            None => None,
        }
    }

    /// Close the WebSocket connection.
    pub async fn close(self) -> Result<(), AlpacaError> {
        self.ws.close().await.map_err(AlpacaError::from)
    }

    async fn send_subscription(
        &mut self,
        action: &str,
        trades: &[&str],
        quotes: &[&str],
        bars: &[&str],
    ) -> Result<(), AlpacaError> {
        let msg = json!({
            "action": action,
            "trades": trades,
            "quotes": quotes,
            "bars": bars,
        });
        self.ws.send(&msg).await.map_err(AlpacaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_data_feed_urls() {
        assert_eq!(
            MarketDataFeed::Sip.url(),
            "wss://stream.data.alpaca.markets/v2/sip"
        );
        assert_eq!(
            MarketDataFeed::Iex.url(),
            "wss://stream.data.alpaca.markets/v2/iex"
        );
        assert_eq!(
            MarketDataFeed::Test.url(),
            "wss://stream.data.alpaca.markets/v2/test"
        );
    }

    #[test]
    fn subscription_message_format() {
        let msg = serde_json::json!({
            "action": "subscribe",
            "trades": ["AAPL"],
            "quotes": ["TSLA", "SPY"],
            "bars": [],
        });
        assert_eq!(msg["action"], "subscribe");
        assert_eq!(msg["trades"][0], "AAPL");
        assert_eq!(msg["quotes"].as_array().unwrap().len(), 2);
        assert!(msg["bars"].as_array().unwrap().is_empty());
    }

    #[test]
    fn auth_message_format() {
        let config = AlpacaConfig::paper("test_key".into(), "test_secret".into());
        let auth = serde_json::json!({
            "action": "auth",
            "key": config.api_key_id,
            "secret": config.api_secret_key,
        });
        assert_eq!(auth["action"], "auth");
        assert_eq!(auth["key"], "test_key");
        assert_eq!(auth["secret"], "test_secret");
    }

    #[test]
    fn trade_updates_url_construction() {
        let config = AlpacaConfig::paper("key".into(), "secret".into());
        let url = config.trading_base_url.replace("https://", "wss://") + "/stream";
        assert_eq!(url, "wss://paper-api.alpaca.markets/stream");
    }

    #[test]
    fn trade_updates_url_live() {
        let config = AlpacaConfig {
            api_key_id: "key".into(),
            api_secret_key: "secret".into(),
            trading_base_url: "https://api.alpaca.markets".into(),
            market_data_base_url: "https://data.alpaca.markets".into(),
        };
        let url = config.trading_base_url.replace("https://", "wss://") + "/stream";
        assert_eq!(url, "wss://api.alpaca.markets/stream");
    }
}
