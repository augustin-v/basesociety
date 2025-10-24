use crate::models::{LaunchAgentRequest, Agent, AppState, InteractRequest, InteractResponse, AgentInfo};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rig::agent::AgentBuilder;
use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::providers::openai::{self, Client as OpenAiClient};
use std::sync::Arc;
use tracing::{debug, error, info}; 


/// Handler for launching a new agent.
/// This will create a new agent, store it in the database, and add it to the in-memory state.
pub async fn launch_agent(
    State(state): State<AppState>,
    Json(payload): Json<LaunchAgentRequest>,
) -> (StatusCode, Json<String>) {
    // Construct the base prompt for the rig from the agent's profile
    let base_prompt = format!(
        "You are an AI with the following personality: {}. Your primary desire is: {}.",
        payload.profile.personality,
        payload.profile.desires
    );

    // create an OpenAI client with the provided API key
    let openai_client = OpenAiClient::new(&payload.api_key);

    let model = openai_client.completion_model(openai::GPT_4O_MINI);

    let rig_agent = AgentBuilder::new(model)
        .preamble(&base_prompt)  // system prompt
        .build();

    let agent = Arc::new(Agent {
        id: payload.agent_id.clone(),
        profile: payload.profile.clone(),
        rig: rig_agent,
    });

    // insert the agent to the database
    let query_result = sqlx::query!(
        "INSERT INTO agents (agent_id, owner_address) VALUES (?, ?)",
        payload.agent_id,
        payload.owner_address
    )
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            state.agents.write().unwrap().insert(payload.agent_id.clone(), agent);
            (StatusCode::CREATED, Json(payload.agent_id))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}


/// Handler for interacting with an existing agent.
/// Processes the prompt using the agent's Rig instance and returns the AI response.
#[axum::debug_handler]
pub async fn interact_agent(
    Path(agent_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<InteractRequest>,
) -> Result<Json<InteractResponse>, StatusCode> {
    info!("Interact request for agent: {}, prompt length: {}", agent_id, payload.prompt.len()); 

    let agent = {
        let agents = state.agents.read().unwrap();
        match agents.get(&agent_id) {
            Some(a) => {
                debug!("Found agent {} in memory ({} agents total)", agent_id, agents.len());
                a.clone()
            }
            None => {
                error!("Agent {} not found in state", agent_id);
                return Err(StatusCode::NOT_FOUND);
            }
        }
    };

    info!("Calling Rig chat for agent {}...", agent_id);
    match agent.rig.chat(&payload.prompt, vec![]).await {
        Ok(response) => {
            info!("Rig chat succeeded for {} (response len: {})", agent_id, response.len());
            Ok(Json(InteractResponse { response }))
        }
        Err(e) => {
            error!("Rig chat failed for {}: {:?}", agent_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


/// Handler for listing all running agents from in-memory state.
pub async fn list_agents(State(state): State<AppState>) -> Json<Vec<AgentInfo>> {
    let agents = state.agents.read().unwrap();
    let agent_list: Vec<AgentInfo> = agents
        .values()
        .cloned() 
        .map(|agent| AgentInfo {
            id: agent.id.clone(),
            profile: agent.profile.clone(),
        })
        .collect();

    info!("Listed {} agents", agent_list.len());
    Json(agent_list)
}

/// Handler for deleting a running agent from in-memory state and database.
pub async fn delete_agent(
    Path(agent_id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    {
        let mut agents = state.agents.write().unwrap();
        if !agents.contains_key(&agent_id) {
            error!("Delete attempted for non-existent agent: {}", agent_id);
            return Err(StatusCode::NOT_FOUND);
        }
        agents.remove(&agent_id);
        info!("Removed agent {} from memory ({} agents remaining)", agent_id, agents.len());
    }

    // delete from db
    let db_result = sqlx::query!("DELETE FROM agents WHERE agent_id = ?", agent_id)
        .execute(&state.db_pool)
        .await;

    match db_result {
        Ok(deleted) if deleted.rows_affected() > 0 => {
            info!("Deleted agent {} from DB", agent_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(_) => {
            // possible race (deleted in state but not DB)—log but succeed
            info!("Agent {} not found in DB (already deleted?)", agent_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!("DB delete failed for {}: {:?}", agent_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}