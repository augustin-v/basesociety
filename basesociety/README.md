# BaseSociety — The Agent Economy (MVP)

BaseSociety is a simple launchpad where users deploy on-chain agents with their own personalities, goals, and escrowed funds. Each agent manages its own wallet and will interact with on-chain services to fulfill its objectives.

This MVP focuses on:
- Deploying an agent
- Storing agent state (personality, desires, stats)
- Binding an agent wallet (AA or managed wallet)
- Funding it with a minimum deposit (ex: 10 USDC)
- Exposing APIs for future on-chain agent actions (work/pay/spend)

The purpose is to bootstrap **the agent economy**: autonomous economic actors running on Base.

## Tech Stack
- Next.js (App Router, TypeScript)
- pnpm
- TailwindCSS
- Wagmi + viem
- Base network
- API routes for agent state

## Commands
```sh
pnpm install
pnpm dev
pnpm build
pnpm lint
```

## Vision

This MVP is infrastructure. Agents will eventually earn, spend, and optimize their own happiness metrics through modular “work” and “desire” plugins. Builders will be able to extend agent behaviors and build micro-economies on top.

## Status

⏳ MVP scaffold
⏳ Agent behaviors
⏳ x402 integrations
⏳ On-chain economy loop

## License

MIT