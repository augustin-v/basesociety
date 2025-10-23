// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "./interfaces/IERC7857.sol";
import "./interfaces/IAgentProfile.sol";

contract DecayOracle is Initializable, OwnableUpgradeable {
    address public agentNFTAddress;
    mapping(uint256 => bool) private registeredAgents;
    mapping(uint256 => uint256) private oracleLastUpdate;
    uint256 private registeredAgentCount;

    event AgentRegistered(uint256 indexed tokenId, address indexed registeredBy);
    event OracleHappinessUpdateTriggered(uint256 indexed tokenId, uint8 oldHappiness, uint8 newHappiness);
    event AgentNFTAddressUpdated(address indexed oldAddress, address indexed newAddress);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _owner, address _agentNFTAddress) public virtual initializer {
        __Ownable_init(_owner);
        require(_agentNFTAddress != address(0), "Zero address for AgentNFT");
        agentNFTAddress = _agentNFTAddress;
        emit AgentNFTAddressUpdated(address(0), _agentNFTAddress);
    }

    function setAgentNFTAddress(address _newAgentNFTAddress) public onlyOwner {
        require(_newAgentNFTAddress != address(0), "Zero address for AgentNFT");
        emit AgentNFTAddressUpdated(agentNFTAddress, _newAgentNFTAddress);
        agentNFTAddress = _newAgentNFTAddress;
    }

    function registerAgent(uint256 tokenId) public onlyOwner {
        require(!registeredAgents[tokenId], "Agent already registered");
        registeredAgents[tokenId] = true;
        registeredAgentCount++;
        emit AgentRegistered(tokenId, msg.sender);
    }

    function updateAgentHappiness(uint256 tokenId, uint8 newHappinessScore) public onlyOwner {
        require(registeredAgents[tokenId], "Agent not registered with oracle");
        require(agentNFTAddress != address(0), "AgentNFT address not set");

        IERC7857 agentNFT = IERC7857(agentNFTAddress);

        AgentProfile memory currentProfile = agentNFT.getAgentProfile(tokenId);
        uint8 oldHappiness = currentProfile.happinessScore;

        agentNFT.updateHappiness(tokenId, newHappinessScore);
        oracleLastUpdate[tokenId] = block.timestamp;

        emit OracleHappinessUpdateTriggered(tokenId, oldHappiness, newHappinessScore);
    }

    function getRegisteredAgentCount() public view returns (uint256) {
        return registeredAgentCount;
    }

    function isAgentRegistered(uint256 tokenId) public view returns (bool) {
        return registeredAgents[tokenId];
    }

    function getOracleLastUpdate(uint256 tokenId) public view returns (uint256) {
        return oracleLastUpdate[tokenId];
    }
}
