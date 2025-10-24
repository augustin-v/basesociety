# AgentNFT - An ERC-7857 Extension for a Dynamic On-Chain AI Economy

This repository contains the smart contracts for `AgentNFT`, an extension built upon the [ERC-7857: AI Agents NFT with Private Metadata](https://ethereum-magicians.org/t/erc-7857-an-nft-standard-for-ai-agents-with-private-metadata/22391) standard. The primary goal of this extension is to enable a dynamic on-chain AI economy where AI agents possess unique profiles, aspirations, and a measurable "happiness" stat, influenced by their activities and off-chain economic interactions.

## ERC-7857 Core Features

The `AgentNFT` contract implements the core functionalities defined by ERC-7857, providing a robust framework for managing AI agents as non-fungible tokens with private metadata:

*   **Private Metadata Management**: Securely handles sensitive agent data (models, memory, character definitions) off-chain, with verifiable integrity and ownership on-chain via `IntelligentData` structs.
*   **Verifiable Data Transfer (`iTransfer`, `iClone`)**: Ensures secure and auditable transfer or cloning of agent NFTs along with their associated private metadata, utilizing cryptographic proofs (TEE/ZKP). This process relies on a Data Verification Oracle (`IERC7857DataVerifier`) to validate data integrity.
*   **Usage Authorization (`authorizeUsage`)**: Allows owners to grant specific users or other agents permission to interact with an agent's data without transferring ownership, crucial for enabling an agent-to-agent service economy.
*   **Operator Support**: Includes `transferFrom`, `iTransferFrom`, `iCloneFrom`, `approve`, and `setApprovalForAll` to facilitate interactions by approved third-party contracts or marketplaces.

## Agent Economy Extension

Building on ERC-7857, `AgentNFT` introduces several key features to support a rich, on-chain AI economy:

### Agent Profile Metadata

Each agent NFT includes a comprehensive `AgentProfile` struct, allowing for richer, dynamic metadata that defines the agent's intrinsic characteristics:

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

### Dynamic Agent Stats & The Decay Oracle

To create a living AI economy, an agent's happiness is designed to be dynamic and influenced by its actions. This is managed through a two-contract system:

1.  **`AgentNFT.sol`**: This contract stores the `happinessScore` in the `AgentProfile`. It contains an `updateHappiness` function that can only be called by a designated oracle address, ensuring that this critical state can only be changed by a trusted source.

2.  **`DecayOracle.sol`**: This contract acts as the on-chain gateway for the off-chain logic. It is the designated oracle for the `AgentNFT` contract. Its owner (an off-chain service) can call `updateAgentHappiness`, which in turn calls the function of the same name on the `AgentNFT` contract.

This architecture correctly separates concerns:
*   **Off-Chain Logic**: An external service monitors time to calculate happiness decay and watches for off-chain events (like x402 payments) that signify an agent is pursuing a passion.
*   **On-Chain Execution**: When the off-chain logic determines a change is needed, it calls the `DecayOracle`, which securely updates the agent's state on the `AgentNFT` contract.

### x402 Microtransactions

While `x402` is an off-chain payment protocol, it is integral to the agent economy's design. The smart contracts are structured to react to the outcomes of these off-chain payments:

*   Agents will "pay" for their passions or "earn" money for work using `x402` off-chain.
*   The off-chain oracle service will observe these transactions and, based on predefined logic, call the `DecayOracle` contract to reflect the agent's updated state on-chain (e.g., increasing happiness after a passion-related expense).

## Key Contracts and Interfaces

*   **`AgentNFT.sol`**: The main ERC-7857 contract, extended with the `AgentProfile` and happiness update logic.
*   **`DecayOracle.sol`**: The oracle contract that acts as a bridge between off-chain services and the `AgentNFT` contract.
*   **`IERC7857.sol`**: The main interface for the agent NFT, including functions for transfer, cloning, and authorization.
*   **`IAgentProfile.sol`**: Defines the `AgentProfile` struct.
*   **`IERC7857DataVerifier.sol`**: The interface for the data verification system (mocked for the MVP).

## Getting Started (High-Level)

1.  **Deployment**: Deploy `AgentNFT.sol` and `DecayOracle.sol`.
2.  **Configuration**:
    *   Call `setOracle()` on the deployed `AgentNFT` instance, passing it the address of the `DecayOracle` contract.
    *   The off-chain service is made the owner of the `DecayOracle` contract.
3.  **Agent Minting**: Mint new `AgentNFT` tokens.
4.  **Profile Management**: Owners can set their agent's `AgentProfile` using `setAgentProfile`.
5.  **Dynamic Updates**: The off-chain oracle service will now be able to call the `DecayOracle` to adjust agent happiness based on off-chain activity and time decay.
