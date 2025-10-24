use alloy::primitives::{Address, B256, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;  // sol! macro
use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::interval;
use tracing::{error, info};
use alloy::hex;

sol! {
    #[sol(rpc)]
    DecayOracle,
    r#"./abis/DecayOracle.json"#
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let rpc_url = std::env::var("BASE_RPC_URL")
        .context("BASE_RPC_URL not set")?
        .parse()?;
    let oracle_addr_str = std::env::var("DECAY_ORACLE_ADDRESS")
        .context("DECAY_ORACLE_ADDRESS not set")?;
    let oracle_addr = Address::from_str(&oracle_addr_str).context("Invalid address")?;
    let private_key_hex = std::env::var("ORACLE_PRIVATE_KEY")
        .context("ORACLE_PRIVATE_KEY not set")?;
    let db_url = "sqlite:../ai_execution/agents.db";

    // Signer: Hex to bytes -> B256
    let private_key_bytes = hex::decode(private_key_hex.trim_start_matches("0x"))
        .context("Invalid hex key")?;
    let key_b256 = B256::from_slice(&private_key_bytes);
    let signer = PrivateKeySigner::from_bytes(&key_b256)
        .context("Invalid private key")?;

    let provider = ProviderBuilder::new().wallet(signer).connect_http(rpc_url);

    let db_pool = SqlitePool::connect(db_url).await.context("DB connect failed")?;

    let contract = DecayOracle::new(oracle_addr, provider);

    // 60s for demo
    let mut tick = interval(Duration::from_secs(60));
    loop {
        tick.tick().await;
        info!("=== Decay Tick Started ===");

        // Query active agents (dummy last_ts)
        let agents: Vec<(String, i64)> = sqlx::query_as(
            "SELECT agent_id, COALESCE(last_interact_ts, 0) as last_ts FROM agents"
        )
        .fetch_all(&db_pool)
        .await
        .context("Query agents failed")?;

        for (agent_id, last_ts) in agents {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let hours_since = ((now - last_ts) as f64 / 3600.0).max(0.0);

            // mocked current happiness (query on-chain later: contract.getOracleLastUpdate(token_id))
            let current_happiness = 80u8;
            if hours_since > 1.0 {
                let decay = (5.0 * hours_since) as i32;
                let new_happiness = (current_happiness as i32 - decay).max(0) as u8;
                if new_happiness < current_happiness {
                    info!("Agent {} decaying: {} -> {} ({} hours idle)", agent_id, current_happiness, new_happiness, hours_since);

                    // dummy id
                    let token_id = U256::from(agent_id.len() as u64);

                    match contract
                        .updateAgentHappiness(token_id, new_happiness)  // function updateAgentHappiness(uint256 tokenId, uint8 newHappinessScore)
                        .send()
                        .await
                    {
                        Ok(tx_hash) => info!("Updated {} happiness via tx: {:?}", agent_id, tx_hash),
                        Err(e) => error!("Tx failed for {}: {:?}", agent_id, e),
                    }

                    // current_happiness = new_happiness;
                }
            } else {
                info!("Agent {} no decay (recent activity)", agent_id);
            }
        }

        info!("=== Decay Tick Complete ===");
    }
}