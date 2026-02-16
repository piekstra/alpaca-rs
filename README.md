# alpaca-rs

[![CI](https://github.com/piekstra/alpaca-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/piekstra/alpaca-rs/actions/workflows/ci.yml)

Rust SDK, CLI, and MCP server for the [Alpaca Trading API](https://alpaca.markets).

## Architecture

Two-layer design for reusability:

- **`api-client-core`** — Generic async REST + WebSocket client abstractions, pagination helpers, and error types. Reusable by future SDKs for other APIs.
- **`alpaca-sdk`** — Alpaca-specific client, types, and WebSocket streaming built on `api-client-core`.
- **`alpaca-cli`** — Command-line interface for querying accounts, positions, orders, quotes, and bars.
- **`alpaca-mcp`** — MCP server exposing Alpaca operations as tools for Claude (planned).

## Quick Start

### SDK — REST

```rust
use alpaca_sdk::{AlpacaClient, AlpacaConfig};

let config = AlpacaConfig::from_env()?;
let client = AlpacaClient::new(config)?;

// Trading
let account = client.get_account().await?;
let positions = client.list_positions().await?;
let order = client.submit_order("AAPL", 10, "buy", "market", "day", None, false).await?;

// Market Data
let quote = client.get_latest_quote("AAPL").await?;
let trade = client.get_latest_trade("TSLA").await?;
let snapshot = client.get_snapshot("SPY").await?;
let bars = client.get_bars("SOXL", start, end, "1Day", None, None, None).await?;

// Reference Data
let assets = client.get_assets(Some("active"), Some("us_equity")).await?;
let calendar = client.get_calendar(Some(start), Some(end)).await?;
let clock = client.get_clock().await?;
```

### SDK — WebSocket Streaming

```rust
use alpaca_sdk::{AlpacaConfig, AlpacaStream, MarketDataFeed};

let config = AlpacaConfig::from_env()?;

// Real-time market data
let mut stream = AlpacaStream::connect_market_data(&config, MarketDataFeed::Iex).await?;
stream.subscribe_trades(&["AAPL", "TSLA"]).await?;
stream.subscribe_quotes(&["SPY"]).await?;

while let Some(msg) = stream.recv().await {
    println!("{:?}", msg?);
}

// Trade updates (order fills, cancellations)
let mut updates = AlpacaStream::connect_trade_updates(&config).await?;
updates.listen_trade_updates().await?;

while let Some(update) = updates.recv_trade_update().await {
    println!("{:?}", update?);
}
```

### CLI

```bash
# Set credentials
export APCA_API_KEY_ID=your_key
export APCA_API_SECRET_KEY=your_secret

# Run commands
cargo run -p alpaca-cli -- account
cargo run -p alpaca-cli -- quote AAPL
cargo run -p alpaca-cli -- bars SOXL --start 2024-01-01 --end 2024-12-31
cargo run -p alpaca-cli -- positions
cargo run -p alpaca-cli -- orders --status open
cargo run -p alpaca-cli -- clock
```

## Configuration

Set environment variables or use `AlpacaConfig::paper()`:

| Variable | Required | Default |
|----------|----------|---------|
| `APCA_API_KEY_ID` | Yes | - |
| `APCA_API_SECRET_KEY` | Yes | - |
| `APCA_TRADING_BASE_URL` | No | `https://paper-api.alpaca.markets` |
| `APCA_MARKET_DATA_BASE_URL` | No | `https://data.alpaca.markets` |

## API Coverage

### Trading API
- Account details
- Submit / get / list / cancel / cancel all / replace orders
- List / close positions
- List / get assets
- Trading calendar
- Market clock

### Market Data API
- Latest quotes and trades
- Stock snapshots (trade + quote + bars)
- Historical bars with auto-pagination
- Historical trades with auto-pagination
- Supports all timeframes (1Min, 5Min, 15Min, 1Hour, 1Day)

### WebSocket Streaming
- Real-time trades, quotes, and minute bars (IEX / SIP feeds)
- Account trade updates (order fills, cancellations, replacements)

## Development

```bash
cargo build --workspace     # Build all crates
cargo test --workspace      # Run all tests
cargo clippy --workspace -- -D warnings  # Lint
cargo fmt --all             # Format
```

## License

MIT
