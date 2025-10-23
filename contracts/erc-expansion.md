# ERC-7857 Extension for Basesociety / AgentNFT

## Abstract

This extension builds upon [ERC-7857](./erc-7857.md) to enable AI agents as NFTs with richer metadata, economic capabilities, lifecycle management, and interaction hooks. Each agent NFT now has personality, desires, skills, versioning, and optional wallets for economic activity. The extension is fully compatible with ERC-7857’s private metadata and TEE/ZKP verification mechanisms.

---

## Specification

### Agent Profile Metadata

Each agent NFT can store structured metadata representing personality, desires, skills, and optionally a privacy-preserving activity log.

```solidity
struct AgentProfile {
    string personality;          // Textual description of agent personality
    string desires;              // Goals or motivations
    string[] skills;             // List of agent capabilities
    bytes32 activityLogHash;     // Optional hash of off-chain activity log
}

mapping(uint256 => AgentProfile) public agentProfiles;
