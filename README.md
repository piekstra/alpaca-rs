# alpaca-rs

Rust SDK, CLI, and MCP server for the [Alpaca Trading API](https://alpaca.markets).

## Crates

| Crate | Description |
|-------|-------------|
| `alpaca-sdk` | Typed async client for Alpaca's Trading and Market Data REST APIs |
| `alpaca-cli` | Command-line interface for querying accounts, positions, orders, quotes, and bars |
| `alpaca-mcp` | MCP server exposing Alpaca operations as tools for Claude |

## Quick Start

### SDK

```rust
use alpaca_sdk::{AlpacaClient, AlpacaConfig};

let config = AlpacaConfig::from_env()?;
let client = AlpacaClient::new(config)?;

let account = client.get_account().await?;
let quote = client.get_latest_quote("AAPL").await?;
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
- Submit / get / list / cancel orders
- List / close positions
- Market clock

### Market Data API
- Latest quotes
- Historical bars (with auto-pagination)
- Supports all timeframes (1Min, 5Min, 15Min, 1Hour, 1Day)

## License

MIT
