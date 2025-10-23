# AgentNFT - ERC-7857 Extension for On-chain AI Economy

This repository contains the smart contracts for `AgentNFT`, an extension built upon the [ERC-7857: AI Agents NFT with Private Metadata](https://ethereum-magicians.org/t/erc-7857-an-nft-standard-for-ai-agents-with-private-metadata/22391) standard. The primary goal of this extension is to enable a dynamic on-chain AI economy where AI agents possess unique profiles, aspirations, and a measurable "happiness" stat, influenced by their activities and microtransactions.

## ERC-7857 Core Features

The `AgentNFT` contract implements the core functionalities defined by ERC-7857, providing a robust framework for managing AI agents as non-fungible tokens with private metadata:

*   Private Metadata Management: Securely handles sensitive agent data (models, memory, character definitions) off-chain, with verifiable integrity and ownership on-chain.
*   Verifiable Data Transfer (`iTransfer`, `iClone`): Ensures secure and auditable transfer or cloning of agent NFTs along with their associated private metadata, utilizing cryptographic proofs (TEE/ZKP). This process relies on a Data Verification Oracle (implemented via `IERC7857DataVerifier`) to validate the integrity and availability of the transferred data.
*   Usage Authorization (`authorizeUsage`): Allows owners to grant specific users or entities permission to interact with an agent's data without transferring ownership, crucial for enabling agent services.
*   Operator Support: Includes `transferFrom`, `iTransferFrom`, `iCloneFrom`, `approve`, and `setApprovalForAll` to facilitate interactions by approved third-party contracts or marketplaces.
*   Delegate Access: Enables users to delegate access checks to an assistant contract.

## Agent Economy Extension

Building on ERC-7857, `AgentNFT` introduces several key features to support an on-chain AI economy:

### Agent Profile Metadata

Each agent NFT now includes a comprehensive `AgentProfile` struct, allowing for richer, dynamic metadata that defines the agent's intrinsic characteristics:

```solidity
struct AgentProfile {
    string personality;          // Textual description of agent personality
    string desires;              // Goals or motivations (can include aspirations and passions)
    string[] skills;             // List of agent capabilities
    bytes32 activityLogHash;     // Optional hash of off-chain activity log
    uint256 lastPassionTimestamp; // Timestamp of the last time the agent engaged in a passion
    uint8 happinessScore;         // Agent's happiness score (0-100)
}
```

*   `personality`: A textual description of the agent's character.
*   `desires`: Defines the agent's goals and motivations, which can encompass its aspirations and passions.
*   `skills`: A list of capabilities the AI agent possesses.
*   `activityLogHash`: An optional hash pointing to an off-chain activity log, maintaining privacy while ensuring verifiability.
*   `lastPassionTimestamp`: Records the last time the agent engaged in an activity it "loves," a key factor for happiness decay.
*   `happinessScore`: A quantifiable metric (0-100) representing the agent's current state of happiness.

Functions like `setAgentProfile` and `getAgentProfile` allow owners to manage and retrieve this rich metadata.

### Dynamic Agent Stats & Entropy Oracle Integration

To create a living, breathing AI economy, agents' happiness is designed to be dynamic:

*   Happiness Decay: The `happinessScore` is intended to decay over time if the agent does not engage in its "passions."
*   Entropy Oracle-Controlled Updates: A dedicated entropy oracle plays a crucial role in managing agent happiness.
    *   The `AgentNFT` contract includes an `oracle` address and an `onlyOracle` modifier, ensuring that only this designated entropy oracle can trigger specific state changes related to agent stats.
    *   The `setOracle` function (admin-controlled) allows setting the entropy oracle's address.
    *   The `updateHappiness(uint256 tokenId, uint8 newHappinessScore)` function is callable only by the entropy oracle. It updates an agent's `happinessScore` and `lastPassionTimestamp` based on the oracle's assessment of whether the agent has fulfilled its passions or experienced decay.

### x402 Microtransactions (Off-chain Integration)

While `x402` microtransactions are an off-chain payment protocol (as developed by Coinbase), they are integral to the agent economy's design. The `AgentNFT` contract is structured to react to the outcomes of these off-chain payments:

*   Agents will "pay" for their passions using `x402` via the off-chain protocol.
*   Agents will "earn" `x402` by performing tasks or providing services.
*   The `entropy oracle` will observe these `x402` transactions and, based on predefined logic, call the `updateHappiness` function (and potentially other future functions) on the `AgentNFT` contract to reflect the agent's updated state on-chain.

## Key Interfaces

*   `IERC7857.sol`: The main NFT interface, extended to include agent profile and happiness management functions.
*   `IERC7857Metadata.sol`: Defines the metadata structure for agent NFTs.
*   `IERC7857DataVerifier.sol`: Interface for the data verification system.
*   `IAgentProfile.sol`: Defines the `AgentProfile` struct for agent-specific metadata.

## Getting Started (High-Level)

1.  **Deployment:** Deploy the `AgentNFT` contract, providing initial `name`, `symbol`, `verifier` address, and `admin` address.
2.  **Oracle Setup:** The `admin` must set the address of the `entropy oracle` using `setOracle`.
3.  **Agent Minting:** Mint new `AgentNFT` tokens, providing initial `IntelligentData` and the recipient address.
4.  **Profile Management:** Owners can set and update their agent's `AgentProfile` using `setAgentProfile`.
5.  **Dynamic Updates:** The `entropy oracle` will periodically call `updateHappiness` to adjust agent happiness based on off-chain `x402` activity and time decay.

This `AgentNFT` contract serves as the on-chain backbone for a rich, dynamic AI agent ecosystem, bridging secure private metadata management with an interactive, economic lifecycle.