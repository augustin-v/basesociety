-- Create the agents table
CREATE TABLE IF NOT EXISTS agents (
    agent_id TEXT PRIMARY KEY NOT NULL,
    owner_address TEXT NOT NULL
);
