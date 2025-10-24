# TODO: ai_execution Crate

This file outlines the development plan for the `ai_execution` REST API service.

## 1. Project Setup & Dependencies
- [x] Initialize the `ai_execution` crate with necessary dependencies.
- [x] Add `axum` for the web framework.
- [x] Add `tokio` for the async runtime.
- [x] Add `sqlx` for SQLite database interaction.
- [x] Add `serde` for serialization/deserialization.
- [x] Add `tracing` and `tracing-subscriber` for logging.
- [x] Add `rig` for the AI agent logic.
- [ ] Add `alloy` or a similar library for blockchain interactions (for agent tools).

## 2. Core Application Structure
- [x] Create `src/main.rs` to set up the Axum server, state, and routes.
- [x] Define shared application state (`AppState`) to hold the database pool and the collection of running agents (`Arc<RwLock<HashMap<...>>>`).
- [x] Implement database setup logic to connect to `agents.db` and run migrations on startup.

## 3. Data Models
- [x] Create `src/models.rs` (or similar).
- [x] Define the `AgentProfile` struct.
- [x] Define the `Agent` struct to hold its state, profile, and `rig` instance.
- [x] Define the JSON request and response structs for the API endpoints (e.g., `LaunchAgentRequest`, `InteractRequest`, `InteractResponse`).

## 4. API Implementation
- [x] Create `src/handlers.rs` for API endpoint logic.
- [x] Implement `POST /agents` handler:
    - [x] Validate request.
    - [x] Persist agent to the database.
    - [x] Create and store the agent in the in-memory `AppState`.
- [ ] Implement `POST /agents/{agent_id}/interact` handler:
    - [ ] Find the agent in `AppState`.
    - [ ] Use the agent's `rig` instance to process the prompt.
    - [ ] Return the AI's response.
- [ ] Implement `GET /agents` handler:
    - [ ] List all running agents from `AppState`.
- [ ] Implement `DELETE /agents/{agent_id}` handler:
    - [ ] Remove the agent from `AppState`.
    - [ ] Delete the agent from the database.

## 5. Agent Tools
- [ ] Create `src/tools.rs`.
- [ ] Implement a tool to fetch ETH and USDC balances for a given address on the Base network.
- [ ] Design the `rig` integration to make these tools available to the agent during prompt execution.

## 6. Testing
- [ ] Set up an in-memory SQLite database for testing.
- [ ] Write integration tests for all API endpoints.
    - [ ] Test successful agent creation and deletion.
    - [ ] Test agent interaction.
    - [ ] Test error cases (e.g., agent not found).
- [ ] Write unit tests for specific business logic (e.g., tool functions).