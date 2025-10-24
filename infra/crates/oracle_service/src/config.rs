use eyre::Result;
use serde::Deserialize;
use std::env;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rpc_url: Url,
    pub agent_nft_address: String,
    pub decay_oracle_address: String,
    pub oracle_service_private_key: String,
    pub database_url: String,
    pub decay_threshold_seconds: u64,
    pub decay_value: i32,
    pub loop_interval_seconds: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv().ok();

        let rpc_url = env::var("RPC_URL")?.parse()?;
        let agent_nft_address = env::var("AGENT_NFT_ADDRESS")?;
        let decay_oracle_address = env::var("DECAY_ORACLE_ADDRESS")?;
        let oracle_service_private_key = env::var("ORACLE_SERVICE_PRIVATE_KEY")?;
        let database_url = env::var("DATABASE_URL")?;
        let decay_threshold_seconds = env::var("DECAY_THRESHOLD_SECONDS")?.parse()?;
        let decay_value = env::var("DECAY_VALUE")?.parse()?;
        let loop_interval_seconds = env::var("LOOP_INTERVAL_SECONDS")?.parse()?;

        Ok(Self {
            rpc_url,
            agent_nft_address,
            decay_oracle_address,
            oracle_service_private_key,
            database_url,
            decay_threshold_seconds,
            decay_value,
            loop_interval_seconds,
        })
    }
}
