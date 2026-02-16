use api_client_core::{paginate, RestClient};
use chrono::NaiveDate;
use reqwest::header::HeaderMap;
use rust_decimal::Decimal;
use tracing::debug;

use crate::config::AlpacaConfig;
use crate::error::AlpacaError;
use crate::types::*;

/// Async client for the Alpaca Trading and Market Data APIs.
///
/// Built on `api_client_core::RestClient` for standardized HTTP handling.
pub struct AlpacaClient {
    trading: RestClient,
    market_data: RestClient,
    config: AlpacaConfig,
}

impl AlpacaClient {
    pub fn new(config: AlpacaConfig) -> Result<Self, AlpacaError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            config
                .api_key_id
                .parse()
                .map_err(|e: reqwest::header::InvalidHeaderValue| {
                    AlpacaError::Config(e.to_string())
                })?,
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            config
                .api_secret_key
                .parse()
                .map_err(|e: reqwest::header::InvalidHeaderValue| {
                    AlpacaError::Config(e.to_string())
                })?,
        );

        let trading = RestClient::builder(&config.trading_base_url)
            .default_headers(headers.clone())
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(AlpacaError::from)?;

        let market_data = RestClient::builder(&config.market_data_base_url)
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(AlpacaError::from)?;

        Ok(Self {
            trading,
            market_data,
            config,
        })
    }

    /// Returns the underlying config (useful for WebSocket auth).
    pub fn config(&self) -> &AlpacaConfig {
        &self.config
    }

    // ── Account ──────────────────────────────────────────────────────

    pub async fn get_account(&self) -> Result<AlpacaAccountResponse, AlpacaError> {
        Ok(self.trading.get("/v2/account").await?)
    }

    // ── Orders ───────────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    pub async fn submit_order(
        &self,
        symbol: &str,
        qty: i32,
        side: &str,
        order_type: &str,
        time_in_force: &str,
        limit_price: Option<Decimal>,
        extended_hours: bool,
    ) -> Result<AlpacaOrderResponse, AlpacaError> {
        let body = AlpacaOrderRequest {
            symbol: symbol.to_string(),
            qty,
            side: side.to_string(),
            order_type: order_type.to_string(),
            time_in_force: time_in_force.to_string(),
            limit_price,
            extended_hours,
        };
        debug!("submit_order symbol={symbol} qty={qty} side={side}");
        Ok(self.trading.post("/v2/orders", &body).await?)
    }

    pub async fn get_order(&self, order_id: &str) -> Result<AlpacaOrderResponse, AlpacaError> {
        Ok(self.trading.get(&format!("/v2/orders/{order_id}")).await?)
    }

    pub async fn list_orders(
        &self,
        status: Option<&str>,
    ) -> Result<Vec<AlpacaOrderResponse>, AlpacaError> {
        let path = match status {
            Some(s) => format!("/v2/orders?status={s}"),
            None => "/v2/orders".to_string(),
        };
        Ok(self.trading.get(&path).await?)
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), AlpacaError> {
        Ok(self
            .trading
            .delete(&format!("/v2/orders/{order_id}"))
            .await?)
    }

    pub async fn cancel_all_orders(&self) -> Result<(), AlpacaError> {
        Ok(self.trading.delete("/v2/orders").await?)
    }

    pub async fn replace_order(
        &self,
        order_id: &str,
        qty: Option<i32>,
        limit_price: Option<Decimal>,
        time_in_force: Option<&str>,
    ) -> Result<AlpacaOrderResponse, AlpacaError> {
        let body = AlpacaReplaceOrderRequest {
            qty,
            limit_price,
            time_in_force: time_in_force.map(|s| s.to_string()),
        };
        Ok(self
            .trading
            .patch(&format!("/v2/orders/{order_id}"), &body)
            .await?)
    }

    // ── Positions ────────────────────────────────────────────────────

    pub async fn list_positions(&self) -> Result<Vec<AlpacaPositionResponse>, AlpacaError> {
        Ok(self.trading.get("/v2/positions").await?)
    }

    pub async fn close_position(&self, symbol: &str) -> Result<AlpacaOrderResponse, AlpacaError> {
        Ok(self
            .trading
            .delete_parsed(&format!("/v2/positions/{symbol}"))
            .await?)
    }

    // ── Assets ───────────────────────────────────────────────────────

    pub async fn get_assets(
        &self,
        status: Option<&str>,
        asset_class: Option<&str>,
    ) -> Result<Vec<AlpacaAssetResponse>, AlpacaError> {
        let mut query = Vec::new();
        if let Some(s) = status {
            query.push(("status", s));
        }
        if let Some(c) = asset_class {
            query.push(("asset_class", c));
        }
        Ok(self.trading.get_with_query("/v2/assets", &query).await?)
    }

    pub async fn get_asset(&self, symbol: &str) -> Result<AlpacaAssetResponse, AlpacaError> {
        Ok(self.trading.get(&format!("/v2/assets/{symbol}")).await?)
    }

    // ── Calendar & Clock ─────────────────────────────────────────────

    pub async fn get_calendar(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<AlpacaCalendarDay>, AlpacaError> {
        let mut query = Vec::new();
        let start_str;
        let end_str;
        if let Some(s) = start {
            start_str = s.to_string();
            query.push(("start", start_str.as_str()));
        }
        if let Some(e) = end {
            end_str = e.to_string();
            query.push(("end", end_str.as_str()));
        }
        Ok(self.trading.get_with_query("/v2/calendar", &query).await?)
    }

    pub async fn get_clock(&self) -> Result<AlpacaClockResponse, AlpacaError> {
        Ok(self.trading.get("/v2/clock").await?)
    }

    // ── Market Data ──────────────────────────────────────────────────

    pub async fn get_latest_quote(&self, symbol: &str) -> Result<AlpacaQuoteResponse, AlpacaError> {
        Ok(self
            .market_data
            .get(&format!("/v2/stocks/{symbol}/quotes/latest"))
            .await?)
    }

    pub async fn get_latest_trade(&self, symbol: &str) -> Result<AlpacaTradeResponse, AlpacaError> {
        Ok(self
            .market_data
            .get(&format!("/v2/stocks/{symbol}/trades/latest"))
            .await?)
    }

    pub async fn get_snapshot(&self, symbol: &str) -> Result<AlpacaSnapshot, AlpacaError> {
        Ok(self
            .market_data
            .get(&format!("/v2/stocks/{symbol}/snapshot"))
            .await?)
    }

    /// Fetch historical bars for a single symbol with auto-pagination.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_bars(
        &self,
        symbol: &str,
        start: NaiveDate,
        end: NaiveDate,
        timeframe: &str,
        feed: Option<&str>,
        adjustment: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<AlpacaBar>, AlpacaError> {
        let limit = limit.unwrap_or(10000);
        let adjustment = adjustment.unwrap_or("split");
        let feed = feed.unwrap_or("iex");
        let base_path = format!(
            "/v2/stocks/{symbol}/bars?start={start}&end={end}&timeframe={timeframe}&adjustment={adjustment}&feed={feed}&limit={limit}"
        );

        let client = &self.market_data;
        let bars = paginate(|page_token| {
            let mut path = base_path.clone();
            if let Some(ref token) = page_token {
                path.push_str(&format!("&page_token={token}"));
            }
            async move {
                let resp: AlpacaSingleSymbolBarsResponse = client.get(&path).await?;
                Ok((resp.bars, resp.next_page_token))
            }
        })
        .await?;

        Ok(bars)
    }

    /// Fetch historical trades for a single symbol with auto-pagination.
    pub async fn get_trades(
        &self,
        symbol: &str,
        start: NaiveDate,
        end: NaiveDate,
        feed: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<AlpacaTrade>, AlpacaError> {
        let limit = limit.unwrap_or(10000);
        let feed = feed.unwrap_or("iex");
        let base_path =
            format!("/v2/stocks/{symbol}/trades?start={start}&end={end}&feed={feed}&limit={limit}");

        let client = &self.market_data;
        let trades = paginate(|page_token| {
            let mut path = base_path.clone();
            if let Some(ref token) = page_token {
                path.push_str(&format!("&page_token={token}"));
            }
            async move {
                let resp: AlpacaTradesPageResponse = client.get(&path).await?;
                Ok((resp.trades, resp.next_page_token))
            }
        })
        .await?;

        Ok(trades)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_from_config() {
        let config = AlpacaConfig::paper("test_key".into(), "test_secret".into());
        let client = AlpacaClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn client_config_accessor() {
        let config = AlpacaConfig::paper("my_key".into(), "my_secret".into());
        let client = AlpacaClient::new(config).unwrap();
        assert_eq!(client.config().api_key_id, "my_key");
        assert_eq!(client.config().api_secret_key, "my_secret");
        assert_eq!(
            client.config().trading_base_url,
            "https://paper-api.alpaca.markets"
        );
    }

    #[test]
    fn client_live_config() {
        let config = AlpacaConfig {
            api_key_id: "key".into(),
            api_secret_key: "secret".into(),
            trading_base_url: "https://api.alpaca.markets".into(),
            market_data_base_url: "https://data.alpaca.markets".into(),
        };
        let client = AlpacaClient::new(config);
        assert!(client.is_ok());
    }
}
