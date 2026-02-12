use alpaca_sdk::AlpacaClient;
use anyhow::Result;

pub async fn account(client: &AlpacaClient) -> Result<serde_json::Value> {
    let account = client.get_account().await?;
    Ok(serde_json::to_value(account)?)
}

pub async fn positions(client: &AlpacaClient) -> Result<serde_json::Value> {
    let positions = client.list_positions().await?;
    Ok(serde_json::to_value(positions)?)
}

pub async fn orders(client: &AlpacaClient, status: Option<&str>) -> Result<serde_json::Value> {
    let orders = client.list_orders(status).await?;
    Ok(serde_json::to_value(orders)?)
}

pub async fn quote(client: &AlpacaClient, symbol: &str) -> Result<serde_json::Value> {
    let quote = client.get_latest_quote(symbol).await?;
    Ok(serde_json::to_value(quote)?)
}

pub async fn bars(
    client: &AlpacaClient,
    symbol: &str,
    start: chrono::NaiveDate,
    end: chrono::NaiveDate,
    timeframe: &str,
) -> Result<serde_json::Value> {
    let bars = client
        .get_bars(symbol, start, end, timeframe, None, None, None)
        .await?;
    Ok(serde_json::to_value(bars)?)
}

pub async fn clock(client: &AlpacaClient) -> Result<serde_json::Value> {
    let clock = client.get_clock().await?;
    Ok(serde_json::to_value(clock)?)
}
