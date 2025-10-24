use rig::agent::Agent as RigAgent;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use reqwest::Client as ReqwestClient;  // for <reqwest::Client> in generic

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::{Arc, RwLock}};

/// Represents the public profile of an AI agent.
/// This information is used to define the agent's personality and capabilities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentProfile {
    pub personality: String,
    pub desires: String,
    pub skills: Vec<String>,
}

/// The request body for launching a new agent.
/// It contains all the necessary information to initialize an agent.
#[derive(Clone, Debug, Deserialize)]
pub struct LaunchAgentRequest {
    pub agent_id: String,
    pub owner_address: String,
    pub api_key: String,
    pub profile: AgentProfile,
}

/// The request body for interacting with an agent.
#[derive(Clone, Debug, Deserialize)]
pub struct InteractRequest {
    pub prompt: String,
}

/// The response body from an agent interaction.
#[derive(Clone, Debug, Serialize)]
pub struct InteractResponse {
    pub response: String,
}

/// Represents a running agent in the system.
/// This includes its ID, profile, and the underlying `rig` instance.
#[derive(Clone)]
pub struct Agent {
    pub id: String,
    pub profile: AgentProfile,
    pub rig: RigAgent<ResponsesCompletionModel<ReqwestClient>>, 
}

/// Represents the information about a running agent that is returned by the API.
#[derive(Clone, Debug, Serialize)]
pub struct AgentInfo {
    pub id: String,
    pub profile: AgentProfile,
}

/// Shared application state (concrete)
#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub agents: Arc<RwLock<HashMap<String, Arc<Agent>>>>,
}