use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ── Account ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaAccountResponse {
    pub id: String,
    pub account_number: String,
    pub status: String,
    pub currency: String,
    pub buying_power: String,
    pub cash: String,
    pub portfolio_value: String,
    pub equity: String,
    pub last_equity: String,
    pub long_market_value: String,
    pub short_market_value: String,
    pub initial_margin: String,
    pub maintenance_margin: String,
    pub daytrade_count: i32,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub shorting_enabled: bool,
    pub multiplier: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub sma: Option<String>,
    #[serde(default)]
    pub crypto_status: Option<String>,
}

// ── Orders ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaOrderResponse {
    pub id: String,
    pub client_order_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: Option<String>,
    pub symbol: String,
    pub asset_class: Option<String>,
    pub notional: Option<String>,
    pub qty: String,
    pub filled_qty: Option<String>,
    pub filled_avg_price: Option<String>,
    pub order_class: Option<String>,
    #[serde(rename = "order_type")]
    pub order_type: Option<String>,
    #[serde(rename = "type")]
    pub type_alias: Option<String>,
    pub side: String,
    pub time_in_force: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<AlpacaOrderResponse>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AlpacaOrderRequest {
    pub symbol: String,
    pub qty: i32,
    pub side: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub time_in_force: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Decimal>,
    pub extended_hours: bool,
}

// ── Positions ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaPositionResponse {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub qty: String,
    pub avg_entry_price: String,
    pub side: String,
    pub market_value: Option<String>,
    pub cost_basis: String,
    pub unrealized_pl: Option<String>,
    pub unrealized_plpc: Option<String>,
    pub unrealized_intraday_pl: Option<String>,
    pub unrealized_intraday_plpc: Option<String>,
    pub current_price: Option<String>,
    pub lastday_price: Option<String>,
    pub change_today: Option<String>,
    pub qty_available: Option<String>,
}

// ── Quotes ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaQuoteResponse {
    pub symbol: Option<String>,
    pub quote: AlpacaQuote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaQuote {
    #[serde(rename = "ap")]
    pub ask_price: Decimal,
    #[serde(rename = "as")]
    pub ask_size: i32,
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    #[serde(rename = "bp")]
    pub bid_price: Decimal,
    #[serde(rename = "bs")]
    pub bid_size: i32,
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    #[serde(rename = "c", default)]
    pub conditions: Option<Vec<String>>,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "z")]
    pub tape: String,
}

// ── Bars ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaBarsResponse {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub bars: std::collections::HashMap<String, Vec<AlpacaBar>>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaSingleSymbolBarsResponse {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub bars: Vec<AlpacaBar>,
    pub symbol: Option<String>,
    pub next_page_token: Option<String>,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaBar {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "v")]
    pub volume: i64,
}

// ── Clock ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaClockResponse {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}
