# alpaca-rs

Rust SDK, CLI, and MCP server for the Alpaca Trading and Market Data APIs.

## Architecture

Two-layer design:

- **`api-client-core`** — Generic async REST + WebSocket client abstractions. Reusable by future SDKs for other APIs.
- **`alpaca-sdk`** — Alpaca-specific client, types, and streaming built on `api-client-core`.
- **`alpaca-cli`** — CLI wrapper around the SDK.
- **`alpaca-mcp`** — MCP server (planned).

## Commands

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all

# Format check (CI)
cargo fmt --all -- --check
```

## Testing Patterns

- Unit tests are inline (`#[cfg(test)] mod tests`) in each module.
- Types tests verify serde deserialization with known JSON payloads from the Alpaca API.
- Pagination tests use closures to simulate multi-page responses.
- No external service calls in unit tests.

## Configuration

All credentials come from environment variables:

- `APCA_API_KEY_ID` — API key (required)
- `APCA_API_SECRET_KEY` — API secret (required)
- `APCA_TRADING_BASE_URL` — Trading API URL (default: paper)
- `APCA_MARKET_DATA_BASE_URL` — Market data URL (default: data.alpaca.markets)

## Key Patterns

- `rust_decimal::Decimal` for all prices (never f64)
- `chrono::DateTime<Utc>` for timestamps
- `thiserror` for error enums with `#[from]` conversions
- `api_client_core::paginate()` for auto-pagination of list endpoints
- `AlpacaStream` wraps `WebSocketClient` for typed market data streaming
