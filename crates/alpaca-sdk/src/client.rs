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

    /// Account equity time series, e.g. period "5A" with timeframe "1D" for
    /// five years of daily closes. Read-only.
    pub async fn get_portfolio_history(
        &self,
        period: &str,
        timeframe: &str,
    ) -> Result<AlpacaPortfolioHistoryResponse, AlpacaError> {
        Ok(self
            .trading
            .get_with_query(
                "/v2/account/portfolio/history",
                &[("period", period), ("timeframe", timeframe)],
            )
            .await?)
    }

    /// List all account activities of the given comma-separated types
    /// (e.g. "CSD,CSW" for cash deposits/withdrawals), oldest first,
    /// paginating until exhausted. Read-only.
    pub async fn get_account_activities(
        &self,
        activity_types: &str,
    ) -> Result<Vec<AlpacaActivityResponse>, AlpacaError> {
        const PAGE_SIZE: usize = 100;
        let activities = paginate(|page_token| {
            let types = activity_types.to_string();
            async move {
                let size = PAGE_SIZE.to_string();
                let mut query: Vec<(&str, &str)> = vec![
                    ("activity_types", types.as_str()),
                    ("direction", "asc"),
                    ("page_size", size.as_str()),
                ];
                if let Some(ref token) = page_token {
                    query.push(("page_token", token.as_str()));
                }
                let page: Vec<AlpacaActivityResponse> = self
                    .trading
                    .get_with_query("/v2/account/activities", &query)
                    .await?;
                // Activities paginate by last-seen id; a short page is the end.
                let next = if page.len() == PAGE_SIZE {
                    page.last().map(|a| a.id.clone())
                } else {
                    None
                };
                Ok((page, next))
            }
        })
        .await?;
        Ok(activities)
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

    /// List orders with full filtering (date range, limit, symbols).
    ///
    /// Alpaca API supports: status ("open"/"closed"/"all"), limit (max 500),
    /// after/until (RFC3339 timestamps), direction ("asc"/"desc"), symbols (comma-separated).
    pub async fn list_orders_filtered(
        &self,
        status: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
        until: Option<&str>,
        direction: Option<&str>,
        symbols: Option<&str>,
    ) -> Result<Vec<AlpacaOrderResponse>, AlpacaError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(s) = status {
            query.push(("status", s.to_string()));
        }
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }
        if let Some(u) = until {
            query.push(("until", u.to_string()));
        }
        if let Some(d) = direction {
            query.push(("direction", d.to_string()));
        }
        if let Some(s) = symbols {
            query.push(("symbols", s.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        Ok(self
            .trading
            .get_with_query("/v2/orders", &query_refs)
            .await?)
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

    // ── Options (read-only) ──────────────────────────────────────────

    /// List option contracts for an underlying from the trading API's
    /// reference data (`GET {trading_base}/v2/options/contracts`).
    ///
    /// `underlying_symbol` is forwarded as the `underlying_symbols` query
    /// param, so a comma-separated list is also accepted. `filter` carries
    /// the optional expiration/strike/type/style/status/limit constraints.
    ///
    /// Pagination follows `next_page_token` but is bounded: at most
    /// `max_pages` pages are fetched (a value of 0 is treated as 1, so at
    /// least one page is always returned).
    pub async fn get_option_contracts(
        &self,
        underlying_symbol: &str,
        filter: &OptionContractsFilter,
        max_pages: usize,
    ) -> Result<Vec<AlpacaOptionContract>, AlpacaError> {
        let cap = max_pages.max(1);
        let mut all: Vec<AlpacaOptionContract> = Vec::new();
        let mut page_token: Option<String> = None;

        for _ in 0..cap {
            let mut query: Vec<(&str, String)> =
                vec![("underlying_symbols", underlying_symbol.to_string())];
            if let Some(d) = filter.expiration_date {
                query.push(("expiration_date", d.to_string()));
            }
            if let Some(d) = filter.expiration_date_gte {
                query.push(("expiration_date_gte", d.to_string()));
            }
            if let Some(d) = filter.expiration_date_lte {
                query.push(("expiration_date_lte", d.to_string()));
            }
            if let Some(s) = filter.strike_price_gte {
                query.push(("strike_price_gte", s.to_string()));
            }
            if let Some(s) = filter.strike_price_lte {
                query.push(("strike_price_lte", s.to_string()));
            }
            if let Some(ref t) = filter.contract_type {
                query.push(("type", t.clone()));
            }
            if let Some(ref s) = filter.style {
                query.push(("style", s.clone()));
            }
            if let Some(ref s) = filter.status {
                query.push(("status", s.clone()));
            }
            if let Some(l) = filter.limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(ref t) = page_token {
                query.push(("page_token", t.clone()));
            }
            let query_refs: Vec<(&str, &str)> =
                query.iter().map(|(k, v)| (*k, v.as_str())).collect();

            let resp: AlpacaOptionContractsResponse = self
                .trading
                .get_with_query("/v2/options/contracts", &query_refs)
                .await?;
            all.extend(resp.option_contracts);

            match resp.next_page_token {
                Some(t) if !t.is_empty() => page_token = Some(t),
                _ => break,
            }
        }

        Ok(all)
    }

    /// Fetch latest option snapshots (quote, trade, greeks, IV) for an
    /// underlying from the market-data API.
    ///
    /// Endpoint: `GET {market_data_base}/v1beta1/options/snapshots/{underlying_symbol}`
    /// (v1beta1, OPRA feed by default). Verified against Alpaca's option-chain
    /// reference — <https://docs.alpaca.markets/reference/optionchain> — on
    /// 2026-08-28. The response is keyed by OCC contract symbol.
    ///
    /// Pagination follows `next_page_token` but is bounded: at most
    /// `max_pages` pages are fetched (a value of 0 is treated as 1). Later
    /// pages are merged into the returned map.
    pub async fn get_option_snapshots(
        &self,
        underlying_symbol: &str,
        filter: &OptionSnapshotsFilter,
        max_pages: usize,
    ) -> Result<std::collections::HashMap<String, AlpacaOptionSnapshot>, AlpacaError> {
        let cap = max_pages.max(1);
        let path = format!("/v1beta1/options/snapshots/{underlying_symbol}");
        let mut all: std::collections::HashMap<String, AlpacaOptionSnapshot> =
            std::collections::HashMap::new();
        let mut page_token: Option<String> = None;

        for _ in 0..cap {
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(ref f) = filter.feed {
                query.push(("feed", f.clone()));
            }
            if let Some(ref t) = filter.contract_type {
                query.push(("type", t.clone()));
            }
            if let Some(s) = filter.strike_price_gte {
                query.push(("strike_price_gte", s.to_string()));
            }
            if let Some(s) = filter.strike_price_lte {
                query.push(("strike_price_lte", s.to_string()));
            }
            if let Some(d) = filter.expiration_date {
                query.push(("expiration_date", d.to_string()));
            }
            if let Some(d) = filter.expiration_date_gte {
                query.push(("expiration_date_gte", d.to_string()));
            }
            if let Some(d) = filter.expiration_date_lte {
                query.push(("expiration_date_lte", d.to_string()));
            }
            if let Some(ref r) = filter.root_symbol {
                query.push(("root_symbol", r.clone()));
            }
            if let Some(ref u) = filter.updated_since {
                query.push(("updated_since", u.clone()));
            }
            if let Some(l) = filter.limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(ref t) = page_token {
                query.push(("page_token", t.clone()));
            }
            let query_refs: Vec<(&str, &str)> =
                query.iter().map(|(k, v)| (*k, v.as_str())).collect();

            let resp: AlpacaOptionSnapshotsResponse =
                self.market_data.get_with_query(&path, &query_refs).await?;
            all.extend(resp.snapshots);

            match resp.next_page_token {
                Some(t) if !t.is_empty() => page_token = Some(t),
                _ => break,
            }
        }

        Ok(all)
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

    #[test]
    fn option_filters_default_empty() {
        let f = OptionContractsFilter::default();
        assert!(f.expiration_date.is_none());
        assert!(f.strike_price_gte.is_none());
        assert!(f.limit.is_none());
        let s = OptionSnapshotsFilter::default();
        assert!(s.feed.is_none());
        assert!(s.contract_type.is_none());
    }

    // ── Live paper tests (opt-in) ────────────────────────────────────
    // Run with creds in the environment:
    //   APCA_API_KEY_ID=… APCA_API_SECRET_KEY=… \
    //     cargo test -p alpaca-sdk -- --ignored --nocapture
    // Never hardcode or print credentials.

    #[tokio::test]
    #[ignore = "hits Alpaca paper; requires APCA_API_KEY_ID/APCA_API_SECRET_KEY"]
    async fn live_get_option_contracts() {
        let config = AlpacaConfig::from_env().expect("env creds");
        let client = AlpacaClient::new(config).unwrap();
        let filter = OptionContractsFilter {
            limit: Some(10),
            ..Default::default()
        };
        let contracts = client
            .get_option_contracts("AAPL", &filter, 1)
            .await
            .expect("fetch contracts");
        assert!(!contracts.is_empty(), "expected at least one AAPL contract");
        let c = &contracts[0];
        assert_eq!(c.underlying_symbol, "AAPL");
        assert!(c.contract_type == "call" || c.contract_type == "put");
    }

    #[tokio::test]
    #[ignore = "hits Alpaca paper; requires APCA_API_KEY_ID/APCA_API_SECRET_KEY"]
    async fn live_get_option_snapshots() {
        let config = AlpacaConfig::from_env().expect("env creds");
        let client = AlpacaClient::new(config).unwrap();
        let filter = OptionSnapshotsFilter {
            limit: Some(10),
            ..Default::default()
        };
        let snaps = client
            .get_option_snapshots("AAPL", &filter, 1)
            .await
            .expect("fetch snapshots");
        assert!(!snaps.is_empty(), "expected at least one AAPL snapshot");
    }
}
