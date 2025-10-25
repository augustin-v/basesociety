use chrono::{DateTime, Utc};
use futures::channel::oneshot;
use reqwest::Client as ReqwestClient; // for <reqwest::Client> in generic
use rig::agent::Agent as RigAgent;
use rig::providers::openai::responses_api::{ResponsesCompletionModel, Role};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::{Mutex, mpsc};

/// Represents the public profile of an AI agent.
/// This information is used to define the agent's personality and capabilities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentProfile {
    pub personality: String,
    pub desires: String,
    pub skills: Vec<String>,
    pub name: String
}

/// The request body for launching a new agent.
/// It contains all the necessary information to initialize an agent.
#[derive(Clone, Debug, Deserialize)]
pub struct LaunchAgentRequest {
    pub agent_id: String,
    pub owner_address: String,
    pub token_id: String,
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

// Custom message with origin and timestamp for persistent history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomMessage {
    pub role: Role,
    pub content: String,
    pub origin: Origin,
    pub timestamp: DateTime<Utc>,
}

/// Origin of a message in agent history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Origin {
    Agent,
    Owner,
    System,
}

/// Commands to inject into an agent's channel.
#[derive(Debug)]
pub enum ChatCommand {
    AddMessage(CustomMessage),
    GetHistory {
        tx: oneshot::Sender<Vec<CustomMessage>>,
    },
    Reflect,
}

/// Updated Agent struct with history and command channel.
#[derive(Clone)]
pub struct Agent {
    pub id: String,
    pub profile: AgentProfile,
    pub rig: RigAgent<ResponsesCompletionModel<ReqwestClient>>,
    pub history: Arc<Mutex<Vec<CustomMessage>>>,
    pub cmd_tx: mpsc::Sender<ChatCommand>,
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
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AgentDetails {
  pub agent_id: String,
  pub profile: AgentProfile,
}