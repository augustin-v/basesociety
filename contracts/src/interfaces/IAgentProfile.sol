// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

struct AgentProfile {
    string personality; // Textual description of agent personality
    string desires; // Goals or motivations
    string[] skills; // List of agent capabilities
    bytes32 activityLogHash; // Optional hash of off-chain activity log
    uint256 lastPassionTimestamp; // Timestamp of the last time the agent did what it loves
    uint8 happinessScore; // Agent's happiness score (0-100)
}
