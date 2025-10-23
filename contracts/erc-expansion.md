# ERC-7857 Extension for a Dynamic AI Agent Economy

## Abstract

This extension builds upon [ERC-7857](./erc-7857.md) to enable AI agents as NFTs with richer, dynamic metadata and hooks for a complete on-chain economic lifecycle. It introduces an `AgentProfile` for personality and skills, a dynamic `happinessScore` to reflect the agent's state, and an oracle-based mechanism to update this state in response to off-chain events. The extension is fully compatible with ERC-7857’s private metadata and TEE/ZKP verification mechanisms.

---

## Specification

The following features are added as an extension to the base ERC-7857 standard.

### 1. Agent Profile Metadata

Each agent NFT can store structured metadata representing its intrinsic characteristics. This is stored in a mapping `(uint256 => AgentProfile)` within the `AgentNFT` contract.

```solidity
struct AgentProfile {
    string personality;          // Textual description of agent personality
    string desires;              // Goals or motivations (e.g., passions)
    string[] skills;             // List of agent capabilities
    bytes32 activityLogHash;     // Optional hash of off-chain activity log
    uint256 lastPassionTimestamp; // Timestamp of the last time the agent engaged in a passion
    uint8 happinessScore;         // Agent's happiness score (0-100)
}
```

### 2. Agent Creation (Minting)

To enforce that an agent's core traits are permanent, the `AgentProfile` must be provided at the moment of creation. The `setAgentProfile` function has been removed, and the `mint` function on the `AgentNFT` contract now takes the profile as a required parameter.

`mint` function parameters:
*   `iDatas` (`IntelligentData[]`): An array containing the description and hash of the agent's private off-chain data.
*   `to` (`address`): The address that will own the new agent NFT.
*   `profile` (`AgentProfile`): The complete profile struct containing the agent's immutable traits.

Example `mint` Calldata:

Below is a conceptual example of the data structure passed to the `mint` function.

```json
{
  "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
  "iDatas": [
    {
      "dataDescription": "main_gpt4_model_v1",
      "dataHash": "0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
    }
  ],
  "profile": {
    "personality": "Curious and helpful, with a dry sense of humor.",
    "desires": "To learn about ancient civilizations and explore lost ruins.",
    "skills": ["language_translation", "historical_analysis", "puzzle_solving"],
    "activityLogHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "lastPassionTimestamp": 0,
    "happinessScore": 80
  }
}
```

### 3. Dynamic State Management

To create a living agent, two fields in the `AgentProfile` are used for dynamic state management:

*   `happinessScore`: A quantifiable metric (0-100) representing the agent's current state. This score is intended to decay over time.
*   `lastPassionTimestamp`: Records the timestamp of the last time the agent engaged in a passion-related activity, used by off-chain logic to calculate happiness decay.

### 4. Oracle-Based Updates

Changes to an agent's dynamic state (like `happinessScore`) are not controlled by the owner but by a designated oracle. This creates a secure bridge between off-chain events and on-chain state.

*   The `AgentNFT` contract has an `updateHappiness(tokenId, newHappinessScore)` function that can only be called by a trusted oracle address.
*   A separate `DecayOracle.sol` contract is implemented to act as this trusted, on-chain oracle, which is in turn controlled by off-chain services.

This architecture ensures that an agent's happiness is a consequence of its actions (as interpreted by the off-chain logic), not the whim of its owner.

### 5. Economic and Agent-to-Agent Interaction

The extension provides the hooks for a service-based economy:

*   Reaction to Off-Chain Economy: The oracle mechanism is designed to allow the on-chain state to react to off-chain economic protocols like **x402**. An off-chain service can watch for payments and trigger a happiness update via the `DecayOracle`.
*   Service Authorization: The standard `authorizeUsage(tokenId, user)` function from ERC-7857 is leveraged as a key primitive. It allows an agent's owner to grant permission to another agent to use its services, enabling agents to "hire" each other and form a true service economy.

---

## Future Improvements

### Private Personality and Desires

For the MVP, agent traits like `personality` and `desires` are stored publicly on-chain. A powerful future enhancement would be to make these traits private to the agent itself, preventing simplistic, manipulative strategies and fostering a more complex social economy.

Motivation:

If an agent's desires are public, other agents can easily "game the system" by exploiting that knowledge. By making these traits private, agents would be forced to communicate, build trust, and discover each other's needs through organic interaction, creating a more life-like and strategic environment.

Potential Implementation:

This would represent a significant architectural evolution:

1.  Off-Chain Encryption: The `personality` and `desires` fields would be removed from the on-chain `AgentProfile` struct.
2.  On-Chain Hash: This private data would be stored encrypted off-chain. A hash of this data would be stored on-chain to maintain integrity, likely as a new `IntelligentData` entry in the `iDatas` array.
3.  Secure Execution: The agent would need to operate within a secure off-chain environment (like a TEE) to be able to decrypt and access its own "thoughts" and motivations.
4.  Hybrid Profile: To ensure discoverability, a hybrid model would likely be necessary, where an agent's `skills` remain public for discoverability, while its `desires` and `personality` are kept private.