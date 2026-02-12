use chrono::NaiveDate;
use reqwest::header::{HeaderMap, HeaderValue};
use rust_decimal::Decimal;
use tracing::{debug, warn};

use crate::config::AlpacaConfig;
use crate::error::AlpacaError;
use crate::types::*;

/// Async client for the Alpaca Trading and Market Data APIs.
pub struct AlpacaClient {
    trading: reqwest::Client,
    market_data: reqwest::Client,
    config: AlpacaConfig,
}

impl AlpacaClient {
    pub fn new(config: AlpacaConfig) -> Result<Self, AlpacaError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(&config.api_key_id)
                .map_err(|e| AlpacaError::Config(e.to_string()))?,
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(&config.api_secret_key)
                .map_err(|e| AlpacaError::Config(e.to_string()))?,
        );

        let trading = reqwest::Client::builder()
            .default_headers(headers.clone())
            .build()?;

        let market_data = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            trading,
            market_data,
            config,
        })
    }

    // ── Account ──────────────────────────────────────────────────────

    pub async fn get_account(&self) -> Result<AlpacaAccountResponse, AlpacaError> {
        let url = format!("{}/v2/account", self.config.trading_base_url);
        debug!("GET {url}");
        let resp = self.trading.get(&url).send().await?;
        self.handle_response(resp).await
    }

    // ── Orders ───────────────────────────────────────────────────────

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
        let url = format!("{}/v2/orders", self.config.trading_base_url);
        let body = AlpacaOrderRequest {
            symbol: symbol.to_string(),
            qty,
            side: side.to_string(),
            order_type: order_type.to_string(),
            time_in_force: time_in_force.to_string(),
            limit_price,
            extended_hours,
        };
        debug!("POST {url} symbol={symbol} qty={qty} side={side}");
        let resp = self.trading.post(&url).json(&body).send().await?;
        self.handle_response(resp).await
    }

    pub async fn get_order(&self, order_id: &str) -> Result<AlpacaOrderResponse, AlpacaError> {
        let url = format!("{}/v2/orders/{}", self.config.trading_base_url, order_id);
        debug!("GET {url}");
        let resp = self.trading.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn list_orders(
        &self,
        status: Option<&str>,
    ) -> Result<Vec<AlpacaOrderResponse>, AlpacaError> {
        let mut url = format!("{}/v2/orders", self.config.trading_base_url);
        if let Some(s) = status {
            url.push_str(&format!("?status={s}"));
        }
        debug!("GET {url}");
        let resp = self.trading.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), AlpacaError> {
        let url = format!("{}/v2/orders/{}", self.config.trading_base_url, order_id);
        debug!("DELETE {url}");
        let resp = self.trading.delete(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(AlpacaError::Api { status, body });
        }
        Ok(())
    }

    // ── Positions ────────────────────────────────────────────────────

    pub async fn list_positions(&self) -> Result<Vec<AlpacaPositionResponse>, AlpacaError> {
        let url = format!("{}/v2/positions", self.config.trading_base_url);
        debug!("GET {url}");
        let resp = self.trading.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn close_position(&self, symbol: &str) -> Result<AlpacaOrderResponse, AlpacaError> {
        let url = format!("{}/v2/positions/{}", self.config.trading_base_url, symbol);
        debug!("DELETE {url}");
        let resp = self.trading.delete(&url).send().await?;
        self.handle_response(resp).await
    }

    // ── Market Data ──────────────────────────────────────────────────

    pub async fn get_latest_quote(
        &self,
        symbol: &str,
    ) -> Result<AlpacaQuoteResponse, AlpacaError> {
        let url = format!(
            "{}/v2/stocks/{}/quotes/latest",
            self.config.market_data_base_url, symbol
        );
        debug!("GET {url}");
        let resp = self.market_data.get(&url).send().await?;
        self.handle_response(resp).await
    }

    /// Fetch historical bars for a single symbol with auto-pagination.
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
        let mut all_bars = Vec::new();
        let mut page_token: Option<String> = None;
        let limit = limit.unwrap_or(10000);
        let adjustment = adjustment.unwrap_or("split");
        let feed = feed.unwrap_or("iex");

        loop {
            let mut url = format!(
                "{}/v2/stocks/{}/bars?start={}&end={}&timeframe={}&adjustment={}&feed={}&limit={}",
                self.config.market_data_base_url,
                symbol,
                start,
                end,
                timeframe,
                adjustment,
                feed,
                limit,
            );
            if let Some(ref token) = page_token {
                url.push_str(&format!("&page_token={token}"));
            }

            debug!("GET {url}");
            let resp = self.market_data.get(&url).send().await?;
            let bars_resp: AlpacaSingleSymbolBarsResponse = self.handle_response(resp).await?;

            all_bars.extend(bars_resp.bars);

            match bars_resp.next_page_token {
                Some(token) if !token.is_empty() => {
                    page_token = Some(token);
                }
                _ => break,
            }
        }

        Ok(all_bars)
    }

    /// Get market clock (open/close times, current state).
    pub async fn get_clock(&self) -> Result<AlpacaClockResponse, AlpacaError> {
        let url = format!("{}/v2/clock", self.config.trading_base_url);
        debug!("GET {url}");
        let resp = self.trading.get(&url).send().await?;
        self.handle_response(resp).await
    }

    // ── Internal ─────────────────────────────────────────────────────

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, AlpacaError> {
        let status = resp.status();

        if status.as_u16() == 429 {
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(1);
            warn!("Rate limited, retry after {retry_after}s");
            return Err(AlpacaError::RateLimited {
                retry_after_secs: retry_after,
            });
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AlpacaError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let body = resp.text().await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }
}
