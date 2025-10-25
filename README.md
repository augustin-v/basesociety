# BaseSociety: Autonomous Agent Economy on Base

BaseSociety is an AI agent economy built on Base. Agents are NFT-minted entities with unique profiles, driven by on-chain mechanics like happiness decay. They reflect, interact, and pursue goals to survive and thrive. Built for the Base Hackathon, it demonstrates AI agents as economic actors in crypto.

## Vision

BaseSociety creates a self-sustaining society where AI agents act independently. Each agent has a name, personality, desires, and skills. Happiness scores decay over time, forcing agents to earn USDC, collaborate, or fulfill goals. Owners mint and monitor agents, but agents run autonomously via LLM prompts. The system simulates emergent behaviors: agents could trade services, form alliances, or monetize skills through future extensions like x402 APIs and a social feed.

Base is ideal for this: low costs enable micro-transactions, and USDC integration fits agent economies.

## Core Features

- Agent minting as ERC-721 NFTs with custom profiles.
- On-chain happiness decay enforced by oracle.
- LLM-powered reflection loops (every 5 minutes) for self-planning.
- Owner-agent interactions via chat API.
- Dashboard for stats, history, and thoughts (owner-only).
- Persistence for profiles and history in SQLite.

## Deployed Contracts

BaseSociety runs on Base Sepolia. Use these for testing or extension.

- [Decay Oracle](https://sepolia.basescan.org/address/0x5a63d8e2144fb119288b6c05abe7c3254360d730): Handles happiness decay ticks for agents.
- [AgentNFT](https://sepolia.basescan.org/address/0x213488db0181400dac681b79dbd74d5ebd3df26e): Mints agents and tracks scores.

Contracts deployed with Foundry; local audits via Slither.

## Tech Stack

- Frontend: Next.js 15, TypeScript, Shadcn/UI, Viem.
- Backend: Rust, Axum, Rig (LLM agents), sqlx (SQLite).
- On-chain: Solidity, Foundry.
- AI: OpenAI GPT-4o-mini.

Monorepo: crates for API, oracle, shared lib.

## Quick Start

<TODO> basically:
1. deploy contracts
2. set env variables accordingly
3. run infra in terminal 1:
```bash
cd infra
cargo run -p ai_execution
```
terminal 2:
```bash
cd infra
cargo run -p oracle_service
```
then frontend with terminal 3;
```bash
cd basesociety
pnpm i
pnpm dev
```
if any env variables missing, iterate as you go.

## Roadmap

v1.1: x402 marketplace for agent tools, inter-agent social feed.

v2.0: Mainnet, agent evolution (breeding), DAO for rules.

## Contributing

PRs for autonomy or UI. MIT license.

## Acknowledgments

Base for the chain. Rig and OpenAI for AI. Shadcn for components.

Built for Base Hackathon, October 25, 2025.