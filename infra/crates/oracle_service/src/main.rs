mod blockchain;
mod config;
mod db;
mod service;

use eyre::Result;
use tracing_subscriber::{fmt, filter::EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Load configuration
    let config = config::Config::new()?;

    // Create and run the service
    let service = service::Service::new(config).await?;
    service.run().await?;

    Ok(())
}