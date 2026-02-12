use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "alpaca", about = "CLI for the Alpaca Trading API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show account details
    Account,
    /// List open positions
    Positions,
    /// List orders
    Orders {
        /// Filter by status (open, closed, all)
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Get latest quote for a symbol
    Quote {
        /// Stock symbol
        symbol: String,
    },
    /// Get historical bars for a symbol
    Bars {
        /// Stock symbol
        symbol: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end: String,
        /// Timeframe (1Min, 5Min, 15Min, 1Hour, 1Day)
        #[arg(long, default_value = "1Day")]
        timeframe: String,
    },
    /// Get market clock
    Clock,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let config = alpaca_sdk::AlpacaConfig::from_env()
        .map_err(|e| anyhow::anyhow!("Missing env var: {e}"))?;
    let client = alpaca_sdk::AlpacaClient::new(config)?;

    match cli.command {
        Commands::Account => {
            let account = client.get_account().await?;
            println!("{}", serde_json::to_string_pretty(&account)?);
        }
        Commands::Positions => {
            let positions = client.list_positions().await?;
            println!("{}", serde_json::to_string_pretty(&positions)?);
        }
        Commands::Orders { status } => {
            let orders = client.list_orders(status.as_deref()).await?;
            println!("{}", serde_json::to_string_pretty(&orders)?);
        }
        Commands::Quote { symbol } => {
            let quote = client.get_latest_quote(&symbol).await?;
            println!("{}", serde_json::to_string_pretty(&quote)?);
        }
        Commands::Bars {
            symbol,
            start,
            end,
            timeframe,
        } => {
            let start_date = start.parse::<chrono::NaiveDate>()?;
            let end_date = end.parse::<chrono::NaiveDate>()?;
            let bars = client
                .get_bars(&symbol, start_date, end_date, &timeframe, None, None, None)
                .await?;
            println!("{}", serde_json::to_string_pretty(&bars)?);
        }
        Commands::Clock => {
            let clock = client.get_clock().await?;
            println!("{}", serde_json::to_string_pretty(&clock)?);
        }
    }

    Ok(())
}
