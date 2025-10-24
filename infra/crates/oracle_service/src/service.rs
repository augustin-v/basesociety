use crate::blockchain::Blockchain;
use crate::config::Config;
use crate::db::Db;
use eyre::Result;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use alloy::primitives::U256;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Service {
    db: Db,
    blockchain: Blockchain,
    config: Config,
}

impl Service {
    pub async fn new(config: Config) -> Result<Self> {
        let db = Db::new(&config.database_url).await?;
        let blockchain = Blockchain::new(&config).await?;
        Ok(Self { db, blockchain, config })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting oracle service...");
        self.run_decay_loop().await;
        Ok(())
    }

    async fn run_decay_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.loop_interval_seconds));

        loop {
            interval.tick().await;
            info!("Running happiness decay check...");

            let agent_ids = match self.db.get_all_agent_ids().await {
                Ok(ids) => ids,
                Err(e) => {
                    warn!("Failed to get agent IDs from DB: {}", e);
                    continue;
                }
            };

            for agent_id_str in agent_ids {
                let agent_id = match U256::from_str(&agent_id_str) {
                    Ok(id) => id,
                    Err(e) => {
                        warn!("Failed to parse agent ID '{}': {}", agent_id_str, e);
                        continue;
                    }
                };

                let profile = match self.blockchain.get_agent_profile(agent_id).await {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("Failed to get profile for agent {}: {}", agent_id, e);
                        continue;
                    }
                };

                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                if now - profile.lastPassionTimestamp.to::<u64>() > self.config.decay_threshold_seconds {
                    info!("Decaying happiness for agent {}", agent_id);
                    match self.blockchain.update_agent_happiness(agent_id, self.config.decay_value as u8).await {
                        Ok(_) => info!("Successfully decayed happiness for agent {}", agent_id),
                        Err(e) => warn!("Failed to decay happiness for agent {}: {}", agent_id, e),
                    }
                }
            }
        }
    }
}
