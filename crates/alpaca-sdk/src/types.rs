use chrono::{DateTime, NaiveDate, Utc};
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

#[derive(Debug, Clone, Serialize)]
pub struct AlpacaReplaceOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
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

// ── Assets ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaAssetResponse {
    pub id: String,
    #[serde(rename = "class")]
    pub asset_class: String,
    pub exchange: String,
    pub symbol: String,
    pub name: Option<String>,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    #[serde(default)]
    pub easy_to_borrow: bool,
    #[serde(default)]
    pub fractionable: bool,
    #[serde(default)]
    pub maintenance_margin_requirement: Option<String>,
}

// ── Calendar ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaCalendarDay {
    pub date: NaiveDate,
    pub open: String,
    pub close: String,
    #[serde(default)]
    pub session_open: Option<String>,
    #[serde(default)]
    pub session_close: Option<String>,
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

// ── Trades ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaTradeResponse {
    pub symbol: Option<String>,
    pub trade: AlpacaTrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaTrade {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub size: i64,
    #[serde(rename = "x")]
    pub exchange: String,
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(rename = "c", default)]
    pub conditions: Option<Vec<String>>,
    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaTradesPageResponse {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub trades: Vec<AlpacaTrade>,
    pub symbol: Option<String>,
    pub next_page_token: Option<String>,
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

// ── Snapshot ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaSnapshot {
    #[serde(rename = "latestTrade", default)]
    pub latest_trade: Option<AlpacaTrade>,
    #[serde(rename = "latestQuote", default)]
    pub latest_quote: Option<AlpacaQuote>,
    #[serde(rename = "minuteBar", default)]
    pub minute_bar: Option<AlpacaBar>,
    #[serde(rename = "dailyBar", default)]
    pub daily_bar: Option<AlpacaBar>,
    #[serde(rename = "prevDailyBar", default)]
    pub prev_daily_bar: Option<AlpacaBar>,
}

// ── Clock ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaClockResponse {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

// ── Stream Messages ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "T")]
pub enum AlpacaStreamMessage {
    #[serde(rename = "success")]
    Success { msg: String },
    #[serde(rename = "error")]
    Error { code: i32, msg: String },
    #[serde(rename = "subscription")]
    Subscription {
        trades: Option<Vec<String>>,
        quotes: Option<Vec<String>>,
        bars: Option<Vec<String>>,
    },
    #[serde(rename = "t")]
    Trade(AlpacaStreamTrade),
    #[serde(rename = "q")]
    Quote(AlpacaStreamQuote),
    #[serde(rename = "b")]
    Bar(AlpacaStreamBar),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaStreamTrade {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub size: i64,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "x")]
    pub exchange: String,
    #[serde(rename = "c", default)]
    pub conditions: Option<Vec<String>>,
    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaStreamQuote {
    #[serde(rename = "S")]
    pub symbol: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaStreamBar {
    #[serde(rename = "S")]
    pub symbol: String,
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
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

// ── Trade Updates (Account Stream) ───────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlpacaTradeUpdate {
    pub event: String,
    pub order: AlpacaOrderResponse,
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    pub position_qty: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_account_response() {
        let json = r#"{
            "id": "abc-123",
            "account_number": "PA123",
            "status": "ACTIVE",
            "currency": "USD",
            "buying_power": "100000.00",
            "cash": "50000.00",
            "portfolio_value": "75000.00",
            "equity": "75000.00",
            "last_equity": "74000.00",
            "long_market_value": "25000.00",
            "short_market_value": "0.00",
            "initial_margin": "12500.00",
            "maintenance_margin": "7500.00",
            "daytrade_count": 2,
            "pattern_day_trader": false,
            "trading_blocked": false,
            "transfers_blocked": false,
            "account_blocked": false,
            "shorting_enabled": true,
            "multiplier": "4",
            "created_at": "2024-01-15T10:30:00Z"
        }"#;
        let account: AlpacaAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(account.id, "abc-123");
        assert_eq!(account.daytrade_count, 2);
        assert!(!account.pattern_day_trader);
        assert!(account.shorting_enabled);
        assert!(account.sma.is_none());
        assert!(account.crypto_status.is_none());
    }

    #[test]
    fn deserialize_order_response() {
        let json = r#"{
            "id": "order-1",
            "client_order_id": "client-1",
            "created_at": "2024-06-01T12:00:00Z",
            "symbol": "AAPL",
            "qty": "10",
            "side": "buy",
            "status": "filled",
            "extended_hours": false
        }"#;
        let order: AlpacaOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "order-1");
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, "buy");
        assert_eq!(order.status, "filled");
    }

    #[test]
    fn serialize_order_request() {
        let req = AlpacaOrderRequest {
            symbol: "TSLA".to_string(),
            qty: 5,
            side: "buy".to_string(),
            order_type: "market".to_string(),
            time_in_force: "day".to_string(),
            limit_price: None,
            extended_hours: false,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["symbol"], "TSLA");
        assert_eq!(json["qty"], 5);
        assert_eq!(json["type"], "market");
        assert!(json.get("limit_price").is_none());
    }

    #[test]
    fn serialize_order_request_with_limit() {
        let req = AlpacaOrderRequest {
            symbol: "AAPL".to_string(),
            qty: 1,
            side: "buy".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            limit_price: Some(Decimal::new(15050, 2)),
            extended_hours: true,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["limit_price"], "150.50");
        assert_eq!(json["extended_hours"], true);
    }

    #[test]
    fn serialize_replace_order_request() {
        let req = AlpacaReplaceOrderRequest {
            qty: Some(10),
            limit_price: Some(Decimal::new(200, 0)),
            time_in_force: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["qty"], 10);
        assert_eq!(json["limit_price"], "200");
        assert!(json.get("time_in_force").is_none());
    }

    #[test]
    fn deserialize_position_response() {
        let json = r#"{
            "asset_id": "asset-1",
            "symbol": "SPY",
            "exchange": "ARCA",
            "asset_class": "us_equity",
            "qty": "100",
            "avg_entry_price": "450.25",
            "side": "long",
            "cost_basis": "45025.00"
        }"#;
        let pos: AlpacaPositionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(pos.symbol, "SPY");
        assert_eq!(pos.qty, "100");
        assert_eq!(pos.side, "long");
    }

    #[test]
    fn deserialize_asset_response() {
        let json = r#"{
            "id": "asset-abc",
            "class": "us_equity",
            "exchange": "NASDAQ",
            "symbol": "AAPL",
            "name": "Apple Inc.",
            "status": "active",
            "tradable": true,
            "marginable": true,
            "shortable": true,
            "easy_to_borrow": true,
            "fractionable": true
        }"#;
        let asset: AlpacaAssetResponse = serde_json::from_str(json).unwrap();
        assert_eq!(asset.symbol, "AAPL");
        assert_eq!(asset.asset_class, "us_equity");
        assert!(asset.tradable);
        assert!(asset.fractionable);
    }

    #[test]
    fn deserialize_calendar_day() {
        let json = r#"{
            "date": "2024-06-15",
            "open": "09:30",
            "close": "16:00"
        }"#;
        let day: AlpacaCalendarDay = serde_json::from_str(json).unwrap();
        assert_eq!(day.date, NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());
        assert_eq!(day.open, "09:30");
        assert_eq!(day.close, "16:00");
        assert!(day.session_open.is_none());
    }

    #[test]
    fn deserialize_quote() {
        let json = r#"{
            "ap": "151.00",
            "as": 200,
            "ax": "Q",
            "bp": "150.98",
            "bs": 100,
            "bx": "Q",
            "t": "2024-06-01T14:30:00Z",
            "z": "C"
        }"#;
        let quote: AlpacaQuote = serde_json::from_str(json).unwrap();
        assert_eq!(quote.ask_price, Decimal::new(15100, 2));
        assert_eq!(quote.bid_price, Decimal::new(15098, 2));
        assert_eq!(quote.tape, "C");
    }

    #[test]
    fn deserialize_trade() {
        let json = r#"{
            "t": "2024-06-01T14:30:00Z",
            "p": "150.50",
            "s": 100,
            "x": "V",
            "i": 12345,
            "z": "C"
        }"#;
        let trade: AlpacaTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price, Decimal::new(15050, 2));
        assert_eq!(trade.size, 100);
        assert_eq!(trade.id, 12345);
    }

    #[test]
    fn deserialize_bar() {
        let json = r#"{
            "t": "2024-06-01T14:30:00Z",
            "o": "150.00",
            "h": "152.50",
            "l": "149.50",
            "c": "151.75",
            "v": 50000
        }"#;
        let bar: AlpacaBar = serde_json::from_str(json).unwrap();
        assert_eq!(bar.open, Decimal::new(15000, 2));
        assert_eq!(bar.high, Decimal::new(15250, 2));
        assert_eq!(bar.low, Decimal::new(14950, 2));
        assert_eq!(bar.close, Decimal::new(15175, 2));
        assert_eq!(bar.volume, 50000);
    }

    #[test]
    fn deserialize_null_bars_response() {
        let json = r#"{"bars": null, "next_page_token": null}"#;
        let resp: AlpacaSingleSymbolBarsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.bars.is_empty());
        assert!(resp.next_page_token.is_none());
    }

    #[test]
    fn deserialize_null_trades_response() {
        let json = r#"{"trades": null, "next_page_token": null}"#;
        let resp: AlpacaTradesPageResponse = serde_json::from_str(json).unwrap();
        assert!(resp.trades.is_empty());
    }

    #[test]
    fn deserialize_snapshot() {
        let json = r#"{
            "latestTrade": {
                "t": "2024-06-01T14:30:00Z",
                "p": "150.50",
                "s": 100,
                "x": "V",
                "i": 1,
                "z": "C"
            },
            "latestQuote": {
                "ap": "151.00",
                "as": 200,
                "ax": "Q",
                "bp": "150.98",
                "bs": 100,
                "bx": "Q",
                "t": "2024-06-01T14:30:00Z",
                "z": "C"
            },
            "minuteBar": {
                "t": "2024-06-01T14:30:00Z",
                "o": "150.00",
                "h": "151.00",
                "l": "149.50",
                "c": "150.75",
                "v": 10000
            },
            "dailyBar": {
                "t": "2024-06-01T00:00:00Z",
                "o": "148.00",
                "h": "152.00",
                "l": "147.50",
                "c": "151.50",
                "v": 5000000
            },
            "prevDailyBar": {
                "t": "2024-05-31T00:00:00Z",
                "o": "147.00",
                "h": "149.00",
                "l": "146.00",
                "c": "148.00",
                "v": 4000000
            }
        }"#;
        let snap: AlpacaSnapshot = serde_json::from_str(json).unwrap();
        assert!(snap.latest_trade.is_some());
        assert!(snap.latest_quote.is_some());
        assert!(snap.minute_bar.is_some());
        assert!(snap.daily_bar.is_some());
        assert!(snap.prev_daily_bar.is_some());
        assert_eq!(snap.latest_trade.unwrap().price, Decimal::new(15050, 2));
    }

    #[test]
    fn deserialize_clock_response() {
        let json = r#"{
            "timestamp": "2024-06-01T14:30:00Z",
            "is_open": true,
            "next_open": "2024-06-02T13:30:00Z",
            "next_close": "2024-06-01T20:00:00Z"
        }"#;
        let clock: AlpacaClockResponse = serde_json::from_str(json).unwrap();
        assert!(clock.is_open);
    }

    #[test]
    fn deserialize_stream_success() {
        let json = r#"{"T": "success", "msg": "authenticated"}"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Success { msg } => assert_eq!(msg, "authenticated"),
            _ => panic!("expected Success"),
        }
    }

    #[test]
    fn deserialize_stream_error() {
        let json = r#"{"T": "error", "code": 401, "msg": "not authenticated"}"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Error { code, msg } => {
                assert_eq!(code, 401);
                assert_eq!(msg, "not authenticated");
            }
            _ => panic!("expected Error"),
        }
    }

    #[test]
    fn deserialize_stream_subscription() {
        let json =
            r#"{"T": "subscription", "trades": ["AAPL"], "quotes": ["AAPL", "TSLA"], "bars": []}"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Subscription {
                trades,
                quotes,
                bars,
            } => {
                assert_eq!(trades.unwrap(), vec!["AAPL"]);
                assert_eq!(quotes.unwrap(), vec!["AAPL", "TSLA"]);
                assert!(bars.unwrap().is_empty());
            }
            _ => panic!("expected Subscription"),
        }
    }

    #[test]
    fn deserialize_stream_trade() {
        let json = r#"{
            "T": "t",
            "S": "AAPL",
            "p": "150.50",
            "s": 100,
            "t": "2024-06-01T14:30:00Z",
            "x": "V",
            "z": "C"
        }"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Trade(t) => {
                assert_eq!(t.symbol, "AAPL");
                assert_eq!(t.price, Decimal::new(15050, 2));
            }
            _ => panic!("expected Trade"),
        }
    }

    #[test]
    fn deserialize_stream_quote() {
        let json = r#"{
            "T": "q",
            "S": "TSLA",
            "ap": "250.00",
            "as": 50,
            "ax": "Q",
            "bp": "249.95",
            "bs": 100,
            "bx": "Q",
            "t": "2024-06-01T14:30:00Z",
            "z": "C"
        }"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Quote(q) => {
                assert_eq!(q.symbol, "TSLA");
                assert_eq!(q.ask_price, Decimal::new(25000, 2));
            }
            _ => panic!("expected Quote"),
        }
    }

    #[test]
    fn deserialize_stream_bar() {
        let json = r#"{
            "T": "b",
            "S": "SPY",
            "o": "450.00",
            "h": "451.00",
            "l": "449.50",
            "c": "450.75",
            "v": 100000,
            "t": "2024-06-01T14:30:00Z"
        }"#;
        let msg: AlpacaStreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            AlpacaStreamMessage::Bar(b) => {
                assert_eq!(b.symbol, "SPY");
                assert_eq!(b.volume, 100000);
            }
            _ => panic!("expected Bar"),
        }
    }

    #[test]
    fn deserialize_trade_update() {
        let json = r#"{
            "event": "fill",
            "order": {
                "id": "order-1",
                "created_at": "2024-06-01T12:00:00Z",
                "symbol": "AAPL",
                "qty": "10",
                "side": "buy",
                "status": "filled",
                "extended_hours": false
            },
            "price": "150.50",
            "qty": "10",
            "timestamp": "2024-06-01T12:01:00Z"
        }"#;
        let update: AlpacaTradeUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.event, "fill");
        assert_eq!(update.order.symbol, "AAPL");
        assert_eq!(update.price.as_deref(), Some("150.50"));
    }

    #[test]
    fn bar_roundtrip_serde() {
        let bar = AlpacaBar {
            timestamp: "2024-06-01T14:30:00Z".parse().unwrap(),
            open: Decimal::new(15000, 2),
            high: Decimal::new(15250, 2),
            low: Decimal::new(14950, 2),
            close: Decimal::new(15175, 2),
            volume: 50000,
        };
        let json = serde_json::to_string(&bar).unwrap();
        let parsed: AlpacaBar = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.open, bar.open);
        assert_eq!(parsed.close, bar.close);
        assert_eq!(parsed.volume, bar.volume);
    }
}
