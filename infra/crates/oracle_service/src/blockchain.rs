use crate::config::Config;
use eyre::Result;
use std::str::FromStr;
use std::sync::Arc;

use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    AgentNFT,
    "./abis/AgentNFT.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    DecayOracle,
    "./abis/DecayOracle.json"
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    agent_nft: AgentNFT::AgentNFT<Provider<Http<Client>>>, // AgentNFT only needs a provider for view calls
    decay_oracle: DecayOracle::DecayOracle<Provider<Http<Client>, PrivateKeySigner>>, // DecayOracle needs a provider with a signer for transactions
}

impl Blockchain {
    pub async fn new(config: &Config) -> Result<Self> {
        let wallet: PrivateKeySigner = config.oracle_service_private_key.parse()?;

        let rpc_url = config.rpc_url.clone();
        let provider = ProviderBuilder::new().wallet(wallet.clone()).connect_http(rpc_url);

        let agent_nft_address = Address::from_str(&config.agent_nft_address)?;
        let agent_nft = AgentNFT::new(agent_nft_address, provider.clone());

        let decay_oracle_address = Address::from_str(&config.decay_oracle_address)?;
        let decay_oracle = DecayOracle::new(decay_oracle_address, provider.clone());

        Ok(Self {
            agent_nft,
            decay_oracle,
        })
    }

    pub async fn get_agent_profile(&self, agent_id: U256) -> Result<AgentNFT::AgentProfile> {
        let call = self.agent_nft.getAgentProfile(agent_id);
        let profile = call.call().await?._0;
        Ok(profile)
    }

    pub async fn update_agent_happiness(&self, agent_id: U256, value: u8) -> Result<()> {
        let call = self.decay_oracle.updateAgentHappiness(agent_id, value);
        let _receipt = call.send().await?.get_receipt().await?;
        Ok(())
    }
}
