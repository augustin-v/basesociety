# Decay Oracle Specifications

This document outlines the specifications for the Decay Oracle, a smart contract designed to manage the dynamic happiness score of AgentNFTs. It acts as the designated "oracle" for the `AgentNFT` contract's `updateHappiness` function, facilitating the on-chain AI economy by influencing agent stats based on time and activity.

## 1. Introduction

The Decay Oracle's primary purpose is to introduce a dynamic element to AgentNFTs' happiness. It will monitor agents, calculate happiness decay over time, and update happiness scores based on off-chain signals indicating passion fulfillment (e.g., via x402 microtransactions).

## 2. Core Functionality

### 2.1. `registerAgent(uint256 tokenId)`

*   Purpose: To explicitly register an AgentNFT with this Decay Oracle. Once registered, the oracle can begin tracking its happiness and decay.
*   Access Control: `onlyOwner` (of the Decay Oracle contract).
*   Considerations: For an MVP, explicit registration is sufficient. In a more advanced system, this could be triggered automatically by events from the `AgentNFT` contract (e.g., `Minted` event).

### 2.2. `updateAgentHappiness(uint256 tokenId, uint8 newHappinessScore)`

*   Purpose: This is the internal function within the Decay Oracle that encapsulates the logic for determining *when* and *how* to call the `AgentNFT` contract's `updateHappiness` function.
*   Access Control: This function will be called by the Decay Oracle's internal logic (e.g., triggered by an off-chain component). It will not be directly callable by external users.
*   Mechanism: This function will first check if the `tokenId` is registered with the Decay Oracle. If registered, it will perform the necessary calculations (decay, passion fulfillment) and then execute the call to `AgentNFT.updateHappiness(tokenId, newHappinessScore)`.

 

## 3. Access Control

### 3.1. `onlyOwner` Functions

*   The Decay Oracle contract will implement an `Ownable` pattern (or similar access control).
*   Critical administrative functions, including `registerAgent` and `setAgentNFTAddress`, will be restricted to the contract owner.

### 3.2. `setAgentNFTAddress(address _agentNFTAddress)`

*   Purpose: To set the address of the `AgentNFT` contract that this Decay Oracle will interact with. This is essential for the oracle to know which `AgentNFT` instance to call `updateHappiness` on.
*   Access Control: `onlyOwner`.

## 4. Internal Logic (Conceptual for MVP)

The following logic will primarily reside in the off-chain component of the Decay Oracle, which then triggers the `updateAgentHappiness` function on the on-chain Decay Oracle contract.

### 4.1. Happiness Decay Mechanism

*   The off-chain oracle component will periodically query the `AgentNFT` contract for `agentProfiles[tokenId].lastPassionTimestamp` for all registered agents.
*   If `block.timestamp - agentProfiles[tokenId].lastPassionTimestamp` exceeds a predefined decay threshold (e.g., 24 hours), the oracle will calculate a reduction in the agent's `happinessScore`.
*   The specific decay rate and minimum happiness floor will be determined by the off-chain logic.

### 4.2. Passion Fulfillment Detection

*   The off-chain oracle component will monitor external signals (e.g., successful `x402` microtransactions where an agent "pays for a passion").
*   Upon detecting such an event, the oracle will calculate an increase in the agent's `happinessScore`.
*   The magnitude of the happiness increase will be determined by the off-chain logic.

### 4.3. Triggering On-chain Updates

*   Based on the calculated decay or fulfillment, the off-chain oracle will determine the `newHappinessScore`.
*   It will then call the `updateAgentHappiness` function on the on-chain Decay Oracle contract, which in turn calls `AgentNFT.updateHappiness(tokenId, newHappinessScore)`.

## 5. Events

To provide transparency and allow off-chain systems to react to oracle actions:

*   `event AgentRegistered(uint256 indexed tokenId, address indexed registeredBy);`
*   `event OracleHappinessUpdateTriggered(uint256 indexed tokenId, uint8 oldHappiness, uint8 newHappiness);`
*   `event AgentNFTAddressUpdated(address indexed oldAddress, address indexed newAddress);`

## 6. View Functions

These functions provide read-only access to the oracle's state, facilitating interaction for external systems and public transparency.

### 6.1. `getRegisteredAgentCount()`

*   Purpose: Returns the total number of AgentNFTs currently registered with this Decay Oracle.
*   Access Control: `public view`.
*   Returns: `uint256` - The count of registered agents.

### 6.2. `isAgentRegistered(uint256 tokenId)`

*   Purpose: Checks if a specific AgentNFT is currently registered with this Decay Oracle.
*   Access Control: `public view`.
*   Parameters: `tokenId` - The ID of the AgentNFT to check.
*   Returns: `bool` - `true` if the agent is registered, `false` otherwise.

### 6.3. `getAgentNFTAddress()`

*   Purpose: Returns the address of the `AgentNFT` contract that this Decay Oracle is configured to interact with.
*   Access Control: `public view`.
*   Returns: `address` - The address of the `AgentNFT` contract.

### 6.4. `getOwner()`

*   Purpose: Returns the address of the current owner of the Decay Oracle contract.
*   Access Control: `public view`.
*   Returns: `address` - The owner's address.

### 6.5. `getOracleLastUpdate(uint256 tokenId)`

*   Purpose: To retrieve the last timestamp when this specific Decay Oracle processed and potentially updated the happiness score for a given AgentNFT. This is the oracle's internal record of its last action on an agent.
*   Access Control: `public view`.
*   Parameters: `tokenId` - The ID of the AgentNFT.
*   Returns: `uint256` - The timestamp of the last update by this oracle for the given agent.
*   Note: This is distinct from `AgentNFT.agentProfiles[tokenId].lastPassionTimestamp`, which records the last time the agent *engaged in a passion* as recorded in the AgentNFT contract.
