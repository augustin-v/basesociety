# TODO: oracle_service Crate

This file outlines the development plan for the `oracle_service` background service.

## 1. Project Setup & Dependencies
- [ ] Initialize the `oracle_service` crate with necessary dependencies.
- [ ] Add `alloy` for blockchain interaction.
- [ ] Add `tokio` for the async runtime and timers.
- [ ] Add `sqlx` for SQLite database interaction.
- [ ] Add `serde` for configuration management.
- [ ] Add `tracing` and `tracing-subscriber` for logging.
- [ ] Add `dotenvy` for managing the private key and other secrets.

## 2. Core Application Structure
- [ ] Create `src/main.rs` to set up the configuration, database and blockchain connections, and start the main service loop.
- [ ] Define a `Config` struct in `src/config.rs` to hold all settings (RPC URL, contract addresses, private key, etc.).
- [ ] Implement logic to load configuration from a file or environment variables.

## 3. Blockchain Interaction (`src/blockchain.rs`)
- [ ] Implement a function to create a blockchain provider and a signer/wallet from the configured private key.
- [ ] Create functions to instantiate typed clients for the `DecayOracle` and `AgentNFT` contracts using their ABIs.
- [ ] Implement `get_agent_profile(agent_id)` to call the `AgentNFT` contract and retrieve an agent's full profile.
- [ ] Implement `update_agent_happiness(agent_id, value)` to build, sign, and send a transaction to the `DecayOracle` contract.

## 4. Database Interaction (`src/db.rs`)
- [ ] Implement a function to create and return an `sqlx::SqlitePool` connection pool.
- [ ] Implement `get_all_agent_ids()` to query the `agents` table and return a `Vec<String>` of all agent IDs.

## 5. Service Logic (`src/service.rs`)
- [ ] Implement the main `run_decay_loop` function.
    - [ ] Set up a `tokio::time::interval`.
    - [ ] In the loop, call the database to get all agent IDs.
    - [ ] For each agent, call the blockchain module to get its profile.
    - [ ] Perform the decay calculation: `if now - profile.last_passion_timestamp > THRESHOLD`.
    - [ ] If decay is needed, call the blockchain module to send the update transaction.
    - [ ] Add robust logging for all actions (e.g., checking agent, decay needed, transaction sent, transaction confirmed).
- [ ] Design a placeholder for the x402 event monitoring logic.
    - [ ] This could be a separate async task.
    - [ ] For the MVP, it might be a simple function that can be manually triggered or a basic polling mechanism.

## 6. Error Handling
- [ ] Define custom error types for the application to handle blockchain, database, and configuration errors gracefully.
- [ ] Ensure all fallible operations (I/O, network calls) are properly handled with `?` or `match`.

## 7. Testing
- [ ] Set up a testing environment that can mock blockchain interactions.
- [ ] Write unit tests for business logic, such as the decay calculation.
- [ ] Write integration tests (if feasible) to test the connection to a local testnet (e.g., Anvil) and a test database.
