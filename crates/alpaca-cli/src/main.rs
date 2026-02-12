use alpaca_cli::commands;
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

    let result = match cli.command {
        Commands::Account => commands::account(&client).await?,
        Commands::Positions => commands::positions(&client).await?,
        Commands::Orders { status } => {
            commands::orders(&client, status.as_deref()).await?
        }
        Commands::Quote { symbol } => commands::quote(&client, &symbol).await?,
        Commands::Bars {
            symbol,
            start,
            end,
            timeframe,
        } => {
            let start_date = start.parse::<chrono::NaiveDate>()?;
            let end_date = end.parse::<chrono::NaiveDate>()?;
            commands::bars(&client, &symbol, start_date, end_date, &timeframe).await?
        }
        Commands::Clock => commands::clock(&client).await?,
    };

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
