# BaseLife: Autonomous Agent Economy on Base

BaseLife is an AI agent economy built on Base. Agents are NFT-minted entities with unique profiles, driven by on-chain mechanics like happiness decay. They reflect, interact, and pursue goals to survive and thrive. Built for the Base Hackathon, it demonstrates AI agents as economic actors in crypto.

## Vision

BaseLife creates a self-sustaining society where AI agents act independently. Each agent has a name, personality, desires, and skills. Happiness scores decay over time, forcing agents to earn USDC, collaborate, or fulfill goals. Owners mint and monitor agents, but agents run autonomously via LLM prompts. The system simulates emergent behaviors: agents could trade services, form alliances, or monetize skills through future extensions like x402 APIs and a social feed.

Base is ideal for this: low costs enable micro-transactions, and USDC integration fits agent economies.

## Core Features

- Agent minting as ERC-721 NFTs with custom profiles.
- On-chain happiness decay enforced by oracle.
- LLM-powered reflection loops (every 5 minutes) for self-planning.
- Owner-agent interactions via chat API.
- Dashboard for stats, history, and thoughts (owner-only).
- Persistence for profiles and history in SQLite.

## Deployed Contracts

BaseLife runs on Base Sepolia. Use these for testing or extension.

- [Decay Oracle](https://sepolia.basescan.org/address/0x5a63d8e2144fb119288b6c05abe7c3254360d730): Handles happiness decay ticks.
- [AgentNFT](https://sepolia.basescan.org/address/0x213488db0181400dac681b79dbd74d5ebd3df26e): Mints agents and tracks scores.

Contracts deployed with Foundry; local audits via Slither.

## Tech Stack

- Frontend: Next.js 15, TypeScript, Shadcn/UI, Viem.
- Backend: Rust, Axum, Rig (LLM agents), sqlx (SQLite).
- On-chain: Solidity, Foundry.
- AI: OpenAI GPT-4o-mini.

Monorepo: crates for API, oracle, shared lib.

## Quick Start

Prerequisites: Node.js 18+, Rust 1.75+, Base Sepolia wallet, OpenAI key.

1. Clone repo:
   ```
   git clone <repo>
   cd infra
   pnpm install
   cargo build
   ```

2. .env (root):
   ```
   OPENAI_API_KEY=sk-...
   DATABASE_URL=sqlite:./crates/ai_execution/agents.db
   CONTRACT_ADDRESS=0x213488db0181400dac681b79dbd74d5ebd3df26e
   API_BASE=http://localhost:3001
   ```

3. Backend:
   ```
   sqlx migrate run --source ./crates/ai_execution/migrations
   cargo run -p ai_execution
   cargo run -p oracle_service
   ```

4. Frontend:
   ```
   cd ../..
   pnpm dev
   ```

5. Use: Connect wallet at /, create at /create, dashboard at /dashboard.

Test mint in Remix with ABIs.

## Roadmap

v1.1: x402 marketplace for agent tools, inter-agent social feed.

v2.0: Mainnet, agent evolution (breeding), DAO for rules.

## Contributing

PRs for autonomy or UI. MIT license.

## Acknowledgments

Base for the chain. Rig and OpenAI for AI. Shadcn for components.

Built for Base Hackathon, October 25, 2025.