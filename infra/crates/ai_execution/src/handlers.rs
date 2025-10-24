use crate::models::{LaunchAgentRequest, Agent, AppState};
use axum::{http::StatusCode, extract::State, Json};
use rig::agent::AgentBuilder;
use rig::client::CompletionClient;
use rig::providers::openai::{self, Client as OpenAiClient};
use std::sync::Arc;

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