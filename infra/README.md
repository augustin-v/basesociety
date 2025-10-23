# Project Baselife: Off-Chain Infrastructure Specification

## 1. High-Level Architecture

The off-chain system for Project Baselife is a stateful application responsible for running the AI agents, managing the dynamics of the agent economy (like happiness decay), and providing an interface for interaction. The entire backend will be developed as a cohesive Rust Workspace, ensuring shared logic, types, and consistency across all services.

The architecture is designed with a clear separation of concerns, with different services (crates) handling distinct parts of the system. The core technologies used across the workspace will be Tokio for asynchronous operations, `sqlx` for database interaction, `serde` for data serialization, and `tracing` for structured logging.

## 2. Rust Workspace Structure

The project will be organized as a cargo workspace. This allows us to build and manage multiple related crates within a single repository.

```
baselife-infra/
├── Cargo.toml         # Workspace manifest to define members
├── crates/
│   ├── ai_execution/  # Crate for the AI Agent REST API
│   ├── oracle_service/ # Crate for the Decay Oracle background service
│   └── shared/         # Crate for shared types and utilities
└── config/
    └── development.toml # Example configuration file
```

---

## 3. Crate Breakdown

### Crate 1: `ai_execution`

This crate is the primary execution environment for the agents. It runs a REST API that allows for the management and interaction with individual AI agents.

-   **Purpose:** To host and execute the AI models, serving their responses via an API.
-   **Responsibilities:**
    -   Expose a REST API to launch, terminate, and list agents.
    -   Provide an endpoint (`/agents/{id}/interact`) to proxy prompts to the correct agent.
    -   Manage the in-memory state of all running agents (their `rig` instances, OpenAI clients, etc.).
    -   Persist and read agent ownership data from the shared SQLite database.
-   **Key Technologies:** `actix-web` (or `axum`), `rig`, `sqlx`.

### Crate 2: `oracle_service`

This crate is a background service that acts as the "Dungeon Master" for the on-chain economy. It has no public API and runs as a standalone process.

-   **Purpose:** To monitor the state of the world and trigger on-chain transactions based on rules and events.
-   **Responsibilities:**
    1.  **Happiness Decay Loop:** Periodically (e.g., every hour), it will connect to the blockchain, read the `lastPassionTimestamp` for all agents registered with the `DecayOracle`, and if the decay threshold is met, it will call the `DecayOracle.updateAgentHappiness()` function to decrease the agent's on-chain happiness score.
    2.  **x402 Event Monitoring:** It will monitor for off-chain payment events. When it detects that an agent has successfully "paid for a passion," it will call `DecayOracle.updateAgentHappiness()` to increase its happiness.
    3.  **On-Chain Transaction Submission:** This service will securely hold the private key for the wallet that owns the `DecayOracle.sol` contract. It will use this key to sign and send all necessary transactions.
-   **Key Technologies:** `ethers-rs` (for blockchain interaction), `tokio::time::interval` (for the decay loop).

### Crate 3: `shared`

This is a common library crate used by `ai_execution` and `oracle_service` to avoid code duplication and ensure consistency.

-   **Purpose:** To provide a single source of truth for shared data structures and utilities.
-   **Responsibilities:**
    -   Define shared data structures that mirror the smart contracts (e.g., `AgentProfile`).
    -   Define the application's configuration struct.
    -   Provide common utility functions, custom error types, and database connection logic.
-   **Key Technologies:** `serde`, `ethers-rs` (for types like `Address`), `sqlx`.

---

## 4. External Services & Dependencies

These are external systems that our Rust infrastructure will rely on.

-   **Blockchain Node Provider:** A connection to an EVM-compatible blockchain. (e.g., Infura, Alchemy).
-   **OpenAI API:** The source of intelligence for the agents. Each agent will have its own API key.
-   **Next.js Frontend:** The existing user-facing web application that will interact with the `ai_execution` REST API.

## 5. Configuration

A central configuration file (e.g., `config/development.toml`) will be used to manage settings for all crates.

**Key Configuration Values:**
-   `rpc_url`: The HTTP endpoint for the Blockchain Node Provider.
-   `agent_nft_address`: The deployed address of the `AgentNFT.sol` contract.
-   `decay_oracle_address`: The deployed address of the `DecayOracle.sol` contract.
-   `oracle_service_private_key`: The private key for the wallet that owns the `DecayOracle` contract.
-   `database_url`: The connection string for the SQLite database (e.g., `sqlite:agents.db`).
