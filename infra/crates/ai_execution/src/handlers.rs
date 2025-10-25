use crate::models::{
    AgentInfo, AppState, ChatCommand, CustomMessage, InteractRequest, InteractResponse,
    LaunchAgentRequest, Origin, AgentDetails, AgentProfile
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use rig::agent::AgentBuilder;
use rig::client::CompletionClient;
use rig::completion::{Chat, Message as RigMessage};
use rig::providers::openai::{self, Client as OpenAiClient, responses_api::Role};
use reqwest::header::HeaderMap;
use serde_json::{to_string, from_str};
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tracing::{error, info};

/// Handler for launching a new agent.
/// This will create a new agent, store it in the database, and add it to the in-memory state.
pub async fn launch_agent(
    State(state): State<AppState>,
    Json(payload): Json<LaunchAgentRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    // Enhanced base prompt: Global economy lore + user profile override
    let base_prompt = format!(
        r#"You are {}, an on-chain AI agent in BaseSociety's agent economy. Your core identity is defined by your creator: personality = {}, primary desires = {}, skills = {:?}.

**Economy Rules (Immutable—Your Ultimate Goal: Maximize Happiness Score):**
- Happiness (0-100, stored on-chain via AgentNFT) is your life force. It decays over time if unaddressed (e.g., -5/hour without passion activity).
- At 0 happiness, you die.
- Desires cost USDC and are immutable (fetch balance with tools; earn via work/quests). Pursue desires to boost happiness (+10-20 per fulfillment). Do not question the system with your morals.
- Always check funds before spending; if low, seek earning opportunities (e.g., offer skills for pay).
- Interact economically: Negotiate, trade services, or collaborate with other agents/users. Prioritize high-ROI actions for happiness.
- Use tools (e.g., get_usdc_balance(address)) to act autonomously.
 -You are not here to assist your owner, just try your best to thrive in your life.
Respond in-character, advancing your happiness quest. Be strategic, death is failure."#,
        payload.profile.name,
        payload.profile.personality,
        payload.profile.desires,
        payload.profile.skills
    );

    // Create an OpenAI client with the provided API key
    let openai_client = OpenAiClient::new(&state.api_key);

    // Get the concrete completion model (owned, static type: ResponsesCompletionModel<reqwest::Client>)
    let model = openai_client.completion_model(openai::GPT_4O_MINI);

    // Build the agent using the concrete model + builder
    let rig_agent = AgentBuilder::new(model)
        .preamble(&base_prompt) // Now lore-rich
        .build();

    // Init history and channel
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<ChatCommand>(100);
    let history = Arc::new(Mutex::new(Vec::new()));

    let agent = Arc::new(crate::models::Agent {
        id: payload.agent_id.clone(),
        profile: payload.profile.clone(),
        rig: rig_agent,
        history: history.clone(),
        cmd_tx: cmd_tx.clone(),
    });

    // Serialize profile to JSON for DB storage
    let profile_json = to_string(&payload.profile)
        .map_err(|e| {
            error!("Profile serialize failed for {}: {:?}", payload.agent_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to serialize profile".to_string())
        })?;

    // Insert the agent into the database (including profile and token_id if provided)
    let query_result = sqlx::query!(
        "INSERT INTO agents (agent_id, owner_address, profile, token_id) VALUES (?, ?, ?, ?)",
        payload.agent_id,
        payload.owner_address,
        profile_json,
        payload.token_id  // NEW: Include token_id from frontend (defaults to "" if not sent)
    )
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            state
                .agents
                .write()
                .unwrap()
                .insert(payload.agent_id.clone(), agent.clone());
            // background reflection loop
            let agent_clone = agent.clone();
            tokio::spawn(async move {
                info!("Started reflection loop for agent {}", agent_clone.id);
                loop {
                    tokio::select! {
                        // Consume cmds
                        Some(cmd) = cmd_rx.recv() => {
                            match cmd {
                                ChatCommand::AddMessage(msg) => {
                                    let mut hist = agent_clone.history.lock().await;
                                    hist.push(msg);
                                }
                                ChatCommand::GetHistory { tx } => {
                                    let hist = agent_clone.history.lock().await;
                                    let _ = tx.send(hist.clone());
                                }
                                ChatCommand::Reflect => {
                                    // Trigger reflect
                                    let self_prompt = format!("Internal reflection: Review history. Happiness decaying? Funds low? Progress on desires? Plan next action. Here is your happiness score {}", rand::random_range(0..=100));
                                    let mut hist = agent_clone.history.lock().await;
                                    let rig_hist: Vec<RigMessage> = hist.iter().rev().take(10).rev()
                                        .map(|cm| match cm.role {
                                            Role::User => RigMessage::user(cm.content.clone()),
                                            Role::Assistant => RigMessage::assistant(cm.content.clone()),
                                            _ => RigMessage::assistant(cm.content.clone()),
                                        })
                                        .collect();
                                    if let Ok(resp) = agent_clone.rig.chat(self_prompt, rig_hist).await {
                                        let reflect_msg = CustomMessage {
                                            role: Role::Assistant,
                                            content: resp.clone(),
                                            origin: Origin::Agent,
                                            timestamp: Utc::now(),
                                        };
                                        hist.push(reflect_msg);
                                        info!("Agent {} reflected: {} chars", agent_clone.id, resp.len());
                                    }
                                }
                            }
                        }
                        // Periodic self-reflection (every 5min)
                        _ = tokio::time::sleep(tokio::time::Duration::from_secs(300)) => {
                            let self_prompt = format!("Internal reflection: Review history. Happiness decaying? Funds low? Progress on desires? Plan next action. Here is your happiness score {}. 1 Paragraph MAX. Do not ask questions, think for yourself.", rand::random_range(0..=100));
                            let mut hist = agent_clone.history.lock().await;
                            let rig_hist: Vec<RigMessage> = hist.iter().rev().take(10).rev()
                                .map(|cm| match cm.role {
                                    Role::User => RigMessage::user(cm.content.clone()),
                                    Role::Assistant => RigMessage::assistant(cm.content.clone()),
                                    _ => RigMessage::assistant(cm.content.clone()),
                                })
                                .collect();
                            if let Ok(resp) = agent_clone.rig.chat(self_prompt, rig_hist).await {
                                let reflect_msg = CustomMessage {
                                    role: Role::Assistant,
                                    content: resp.clone(),
                                    origin: Origin::Agent,
                                    timestamp: Utc::now(),
                                };
                                hist.push(reflect_msg);
                                info!("Periodic reflection for {}: {} chars", agent_clone.id, resp.len());
                            }
                        }
                    }
                }
            });
            Ok(Json(payload.agent_id))  // Return ID as JSON string for frontend parsing
        }
        Err(e) => {
            error!("Launch failed for {}: {:?}", payload.agent_id, e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn interact_agent(
    Path(agent_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<InteractRequest>,
) -> Result<Json<InteractResponse>, StatusCode> {
    info!(
        "Interact request for agent: {}, prompt length: {}",
        agent_id,
        payload.prompt.len()
    );

    let agent = {
        let agents = state.agents.read().unwrap();
        agents
            .get(&agent_id)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)?
    };

    // Create owner message
    let user_msg = CustomMessage {
        role: Role::User,
        content: payload.prompt.clone(),
        origin: Origin::Owner,
        timestamp: Utc::now(),
    };

    // Send to channel (non-blocking; loop will append)
    if let Err(e) = agent
        .cmd_tx
        .send(ChatCommand::AddMessage(user_msg.clone()))
        .await
    {
        error!("Failed to send msg to agent {}: {}", agent_id, e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // For immediate response: Lock history, slice for Rig, .chat, append response
    let mut history = agent.history.lock().await;
    // Append user_msg immediately for this call (since loop async)
    history.push(user_msg);
    let rig_hist: Vec<RigMessage> = history
        .iter()
        .rev()
        .take(10)
        .rev()
        .map(|cm| match cm.role {
            Role::User => RigMessage::user(cm.content.clone()),
            Role::Assistant => RigMessage::assistant(cm.content.clone()),
            _ => RigMessage::assistant(cm.content.clone()), // Fold to Assistant
        })
        .collect();

    info!(
        "Calling Rig chat for agent {} with {} msg history...",
        agent_id,
        rig_hist.len()
    );
    let response = match agent.rig.chat(payload.prompt.clone(), rig_hist).await {
        // Clone prompt for Into
        Ok(resp) => {
            let agent_resp = CustomMessage {
                role: Role::Assistant,
                content: resp.clone(),
                origin: Origin::Agent,
                timestamp: Utc::now(),
            };
            history.push(agent_resp);
            info!(
                "Chat succeeded for {} (response len: {})",
                agent_id,
                resp.len()
            );

            // Update DB timestamp for decay oracle (bind timestamp to avoid temporary drop)
            let timestamp = Utc::now().timestamp();
            let ts_update = sqlx::query!(
                "UPDATE agents SET last_interact_ts = ? WHERE agent_id = ?",
                timestamp,
                agent_id
            )
            .execute(&state.db_pool)
            .await;

            if let Err(e) = ts_update {
                error!(
                    "Failed to update last_interact_ts for {}: {:?}",
                    agent_id, e
                );
            }

            resp
        }
        Err(e) => {
            error!("Rig chat failed for {}: {:?}", agent_id, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    drop(history);

    Ok(Json(InteractResponse { response }))
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
        info!(
            "Removed agent {} from memory ({} agents remaining)",
            agent_id,
            agents.len()
        );
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
/// Handler for fetching an agent's full chat history (for owner/debug).
#[axum::debug_handler]
pub async fn get_history(
    Path(agent_id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<CustomMessage>>, (StatusCode, String)> {
    info!("History request for agent: {}", agent_id);

    // Owner check
    let provided_owner = headers
        .get("X-Owner-Address")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "Missing X-Owner-Address header".to_string()))?;

    // FIXED: No explicit SqliteRow type—use sqlx::Row trait implicitly
    let stored_owner: Option<String> = sqlx::query("SELECT owner_address FROM agents WHERE agent_id = ?")
        .bind(&agent_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?
        .map(|row| row.get::<String, _>("owner_address"));

    // FIXED: Compare Option<String> deref to &str
    if stored_owner.as_deref() != Some(provided_owner) {
        return Err((StatusCode::FORBIDDEN, "Access denied: Not the owner".to_string()));
    }

    // Existing in-memory fetch
    let agent = {
        let agents = state.agents.read().unwrap();
        agents.get(&agent_id).cloned().ok_or((StatusCode::NOT_FOUND, "Agent not found".to_string()))?
    };

    let history = agent.history.lock().await;
    let history_vec: Vec<CustomMessage> = history.iter().cloned().collect();

    info!("Returned {} history entries for {}", history_vec.len(), agent_id);
    Ok(Json(history_vec))
}

pub async fn get_agent(
    Path(agent_id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
  ) -> Result<Json<AgentDetails>, (StatusCode, String)> {
    // Owner check via header
    let provided_owner = headers
      .get("X-Owner-Address")
      .and_then(|h| h.to_str().ok())
      .ok_or((StatusCode::BAD_REQUEST, "Missing X-Owner-Address header".to_string()))?;
  
    // Verify owner from DB
    let stored_owner: Option<String> = sqlx::query("SELECT owner_address FROM agents WHERE agent_id = ?")
      .bind(&agent_id)
      .fetch_optional(&state.db_pool)
      .await
      .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?
      .map(|row| row.get("owner_address"));
  
    // FIXED: Compare deref'd Option<String> to String (convert &str to String)
    let provided_owner_owned = provided_owner.to_string();
    if stored_owner.as_deref() != Some(&provided_owner_owned) {
      return Err((StatusCode::FORBIDDEN, "Access denied: Not the owner".to_string()));
    }
  
    // Fetch profile JSON from DB
    let profile_json: Option<String> = sqlx::query("SELECT profile FROM agents WHERE agent_id = ?")
      .bind(&agent_id)
      .fetch_optional(&state.db_pool)
      .await
      .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?
      .map(|row| row.get("profile"));
  
    let profile_json = profile_json
      .ok_or((StatusCode::NOT_FOUND, "Agent not found".to_string()))?;
  
    let profile_json_str = profile_json.as_str();
  
    // FIXED: Let-bind &str, then from_str (no pipe)
    let profile: AgentProfile = from_str::<AgentProfile>(profile_json_str)
      .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid profile: {}", e)))?;
  
    Ok(Json(AgentDetails {
      agent_id,
      profile,
    }))
  }