// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IERC7857.sol";
import "./interfaces/IERC7857Metadata.sol";
import "./interfaces/IERC7857DataVerifier.sol";
import "./Utils.sol";
import {AgentProfile} from "./interfaces/IAgentProfile.sol";

contract AgentNFT is
    AccessControlUpgradeable,
    ReentrancyGuardUpgradeable,
    PausableUpgradeable,
    IERC7857,
    IERC7857Metadata
{
    event Updated(uint256 indexed _tokenId, IntelligentData[] _oldDatas, IntelligentData[] _newDatas);

    event Minted(uint256 indexed _tokenId, address indexed _creator, address indexed _owner);

    struct TokenData {
        address owner;
        address[] authorizedUsers;
        address approvedUser;
        IntelligentData[] iDatas;
    }

    /// @custom:storage-location erc7201:agent.storage.AgentNFT
    struct AgentNFTStorage {
        // Token data
        mapping(uint256 => TokenData) tokens;
        mapping(address owner => mapping(address operator => bool)) operatorApprovals;
        mapping(address user => address accessAssistant) accessAssistants;
        uint256 nextTokenId;
        mapping(uint256 => AgentProfile) agentProfiles; // Agent profile metadata
        // Contract metadata
        string name;
        string symbol;
        // Core components
        IERC7857DataVerifier verifier;
        address admin;
        address oracle; // Address of the entropy oracle
    }

    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    modifier onlyOracle() {
        require(msg.sender == _getAgentStorage().oracle, "Only oracle can call this function");
        _;
    }

    uint256 public constant MAX_AUTHORIZED_USERS = 100;

    string public constant VERSION = "2.0.0";

    // keccak256(abi.encode(uint(keccak256("agent.storage.AgentNFT")) - 1)) & ~bytes32(uint(0xff))
    bytes32 private constant AGENT_NFT_STORAGE_LOCATION =
        0x4aa80aaafbe0e5fe3fe1aa97f3c1f8c65d61f96ef1aab2b448154f4e07594600;

    function _getAgentStorage() private pure returns (AgentNFTStorage storage $) {
        assembly {
            $.slot := AGENT_NFT_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(string memory name_, string memory symbol_, address verifierAddr, address admin_)
        public
        virtual
        initializer
    {
        require(verifierAddr != address(0), "Zero address");
        require(admin_ != address(0), "Invalid admin address");

        __AccessControl_init();
        __ReentrancyGuard_init();
        __Pausable_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin_);
        _grantRole(ADMIN_ROLE, admin_);
        _grantRole(PAUSER_ROLE, admin_);

        AgentNFTStorage storage $ = _getAgentStorage();
        $.name = name_;
        $.symbol = symbol_;
        $.verifier = IERC7857DataVerifier(verifierAddr);
        $.admin = admin_;
    }

    function setAdmin(address newAdmin) external override onlyRole(DEFAULT_ADMIN_ROLE) {
        require(newAdmin != address(0), "Invalid admin address");
        address oldAdmin = _getAgentStorage().admin;

        if (oldAdmin != newAdmin) {
            _getAgentStorage().admin = newAdmin;

            _grantRole(DEFAULT_ADMIN_ROLE, newAdmin);
            _grantRole(ADMIN_ROLE, newAdmin);
            _grantRole(PAUSER_ROLE, newAdmin);

            _revokeRole(DEFAULT_ADMIN_ROLE, oldAdmin);
            _revokeRole(ADMIN_ROLE, oldAdmin);
            _revokeRole(PAUSER_ROLE, oldAdmin);

            emit AdminChanged(oldAdmin, newAdmin);
        }
    }

    /// @notice Sets the address of the entropy oracle. Only callable by an admin.
    /// @param newOracle The address of the new entropy oracle.
    function setOracle(address newOracle) external onlyRole(ADMIN_ROLE) {
        require(newOracle != address(0), "Zero address");
        _getAgentStorage().oracle = newOracle;
    }

    // Basic getters
    function name() public view virtual returns (string memory) {
        return _getAgentStorage().name;
    }

    function symbol() public view virtual returns (string memory) {
        return _getAgentStorage().symbol;
    }

    function verifier() public view virtual returns (IERC7857DataVerifier) {
        return _getAgentStorage().verifier;
    }

    function admin() public view virtual returns (address) {
        return _getAgentStorage().admin;
    }

    // Admin functions
    function updateVerifier(address newVerifier) public virtual onlyRole(ADMIN_ROLE) {
        require(newVerifier != address(0), "Zero address");
        _getAgentStorage().verifier = IERC7857DataVerifier(newVerifier);
    }




    
    /// @notice Updates the happiness score and last passion timestamp of an agent.
    ///         This function is access-controlled and can only be called by the designated oracle.
    /// @param tokenId The token ID of the agent.
    /// @param newHappinessScore The new happiness score (0-100).
    function updateHappiness(uint256 tokenId, uint8 newHappinessScore) public virtual onlyOracle {
        AgentNFTStorage storage $ = _getAgentStorage();
        require(_exists(tokenId), "Token does not exist");
        require(newHappinessScore <= 100, "Happiness score cannot exceed 100");

        $.agentProfiles[tokenId].happinessScore = newHappinessScore;
        $.agentProfiles[tokenId].lastPassionTimestamp = block.timestamp;
    }

    function mint(
        IntelligentData[] calldata iDatas,
        address to,
        AgentProfile calldata profile
    ) public payable virtual returns (uint256 tokenId) {
        require(to != address(0), "Zero address");
        require(iDatas.length > 0, "Empty data array");

        AgentNFTStorage storage $ = _getAgentStorage();

        tokenId = $.nextTokenId++;
        TokenData storage newToken = $.tokens[tokenId];
        newToken.owner = to;
        newToken.approvedUser = address(0);

        for (uint i = 0; i < iDatas.length; i++) {
            newToken.iDatas.push(iDatas[i]);
        }

        $.agentProfiles[tokenId] = profile;

        emit Minted(tokenId, msg.sender, to);
    }

    function _proofCheck(address from, address to, uint256 tokenId, TransferValidityProof[] calldata proofs)
        internal
        returns (bytes[] memory sealedKeys, IntelligentData[] memory newDatas)
    {
        AgentNFTStorage storage $ = _getAgentStorage();
        require(to != address(0), "Zero address");
        require($.tokens[tokenId].owner == from, "Not owner");
        require(proofs.length > 0, "Empty proofs array");

        TransferValidityProofOutput[] memory proofOutput = $.verifier.verifyTransferValidity(proofs);

        require(proofOutput.length == $.tokens[tokenId].iDatas.length, "Proof count mismatch");

        sealedKeys = new bytes[](proofOutput.length);
        newDatas = new IntelligentData[](proofOutput.length);

        for (uint256 i = 0; i < proofOutput.length; i++) {
            // require the initial data hash is the same as the old data hash
            require(proofOutput[i].oldDataHash == $.tokens[tokenId].iDatas[i].dataHash, "Old data hash mismatch");

            // only the receiver itself or the access assistant can sign the access proof
            require(
                proofOutput[i].accessAssistant == $.accessAssistants[to] || proofOutput[i].accessAssistant == to,
                "Access assistant mismatch"
            );

            bytes memory wantedKey = proofOutput[i].wantedKey;
            bytes memory encryptedPubKey = proofOutput[i].encryptedPubKey;
            if (wantedKey.length == 0) {
                // if the wanted key is empty, the default wanted receiver is receiver itself
                address defaultWantedReceiver = Utils.pubKeyToAddress(encryptedPubKey);
                require(defaultWantedReceiver == to, "Default wanted receiver mismatch");
            } else {
                // if the wanted key is not empty, the data is private
                require(Utils.bytesEqual(encryptedPubKey, wantedKey), "encryptedPubKey mismatch");
            }

            sealedKeys[i] = proofOutput[i].sealedKey;
            newDatas[i] = IntelligentData({
                dataDescription: $.tokens[tokenId].iDatas[i].dataDescription,
                dataHash: proofOutput[i].newDataHash
            });
        }
        return (sealedKeys, newDatas);
    }

    function _transfer(address from, address to, uint256 tokenId, TransferValidityProof[] calldata proofs) internal {
        AgentNFTStorage storage $ = _getAgentStorage();
        (bytes[] memory sealedKeys, IntelligentData[] memory newDatas) = _proofCheck(from, to, tokenId, proofs);

        TokenData storage token = $.tokens[tokenId];
        token.owner = to;
        token.approvedUser = address(0);

        // Clear authorized users on transfer
        delete token.authorizedUsers;

        delete token.iDatas;
        for (uint256 i = 0; i < newDatas.length; i++) {
            token.iDatas.push(newDatas[i]);
        }

        emit Transferred(tokenId, from, to);
        emit PublishedSealedKey(to, tokenId, sealedKeys);
    }

    function iTransfer(address to, uint256 tokenId, TransferValidityProof[] calldata proofs) public virtual {
        require(_isApprovedOrOwner(msg.sender, tokenId), "Not authorized");
        _transfer(ownerOf(tokenId), to, tokenId, proofs);
    }

    function transferFrom(address from, address to, uint256 tokenId) public virtual {
        TokenData storage token = _getAgentStorage().tokens[tokenId];
        require(_isApprovedOrOwner(msg.sender, tokenId), "Not authorized");
        require(to != address(0), "Zero address");
        require(token.owner == from, "Not owner");
        token.owner = to;
        token.approvedUser = address(0);

        // Clear authorized users on transfer
        delete token.authorizedUsers;

        emit Transferred(tokenId, from, to);
    }

    function iTransferFrom(address from, address to, uint256 tokenId, TransferValidityProof[] calldata proofs)
        public
        virtual
    {
        require(_isApprovedOrOwner(msg.sender, tokenId), "Not authorized");
        _transfer(from, to, tokenId, proofs);
    }

    function _clone(address from, address to, uint256 tokenId, TransferValidityProof[] calldata proofs)
        internal
        returns (uint256)
    {
        AgentNFTStorage storage $ = _getAgentStorage();

        (bytes[] memory sealedKeys, IntelligentData[] memory newDatas) = _proofCheck(from, to, tokenId, proofs);

        uint256 newTokenId = $.nextTokenId++;
        TokenData storage newToken = $.tokens[newTokenId];
        newToken.owner = to;
        newToken.approvedUser = address(0);

        for (uint256 i = 0; i < newDatas.length; i++) {
            newToken.iDatas.push(newDatas[i]);
        }

        emit Cloned(tokenId, newTokenId, from, to);
        emit PublishedSealedKey(to, newTokenId, sealedKeys);

        return newTokenId;
    }

    function iClone(address to, uint256 tokenId, TransferValidityProof[] calldata proofs)
        public
        virtual
        returns (uint256)
    {
        require(_isApprovedOrOwner(msg.sender, tokenId), "Not authorized");
        return _clone(ownerOf(tokenId), to, tokenId, proofs);
    }

    function iCloneFrom(address from, address to, uint256 tokenId, TransferValidityProof[] calldata proofs)
        public
        virtual
        returns (uint256)
    {
        require(_isApprovedOrOwner(msg.sender, tokenId), "Not authorized");
        return _clone(from, to, tokenId, proofs);
    }

    function authorizeUsage(uint256 tokenId, address to) public virtual {
        require(to != address(0), "Zero address");
        AgentNFTStorage storage $ = _getAgentStorage();
        require($.tokens[tokenId].owner == msg.sender, "Not owner");

        address[] storage authorizedUsers = $.tokens[tokenId].authorizedUsers;

        require(authorizedUsers.length < MAX_AUTHORIZED_USERS, "Too many authorized users");

        for (uint256 i = 0; i < authorizedUsers.length; i++) {
            require(authorizedUsers[i] != to, "Already authorized");
        }

        authorizedUsers.push(to);
        emit Authorization(msg.sender, to, tokenId);
    }

    function ownerOf(uint256 tokenId) public view virtual returns (address) {
        AgentNFTStorage storage $ = _getAgentStorage();
        address owner = $.tokens[tokenId].owner;
        require(owner != address(0), "Token does not exist");
        return owner;
    }

    function authorizedUsersOf(uint256 tokenId) public view virtual returns (address[] memory) {
        AgentNFTStorage storage $ = _getAgentStorage();
        require(_exists(tokenId), "Token does not exist");
        return $.tokens[tokenId].authorizedUsers;
    }

    function _exists(uint256 tokenId) internal view returns (bool) {
        return _getAgentStorage().tokens[tokenId].owner != address(0);
    }

    function intelligentDataOf(uint256 tokenId) public view virtual returns (IntelligentData[] memory) {
        AgentNFTStorage storage $ = _getAgentStorage();
        require(_exists(tokenId), "Token does not exist");
        return $.tokens[tokenId].iDatas;
    }

    /// @notice Retrieves the agent profile for a given token ID.
    /// @param tokenId The token ID of the agent.
    /// @return The AgentProfile of the agent.
    function getAgentProfile(uint256 tokenId) public view virtual returns (AgentProfile memory) {
        AgentNFTStorage storage $ = _getAgentStorage();
        require(_exists(tokenId), "Token does not exist");
        return $.agentProfiles[tokenId];
    }

    function approve(address to, uint256 tokenId) public virtual {
        address owner = ownerOf(tokenId);
        require(to != owner, "Approval to current owner");
        require(msg.sender == owner || isApprovedForAll(owner, msg.sender), "Not authorized");

        _getAgentStorage().tokens[tokenId].approvedUser = to;
        emit Approval(owner, to, tokenId);
    }

    function setApprovalForAll(address operator, bool approved) public virtual {
        require(operator != msg.sender, "Approve to caller");
        _getAgentStorage().operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    function getApproved(uint256 tokenId) public view virtual returns (address) {
        require(_exists(tokenId), "Token does not exist");
        return _getAgentStorage().tokens[tokenId].approvedUser;
    }

    function isApprovedForAll(address owner, address operator) public view virtual returns (bool) {
        return _getAgentStorage().operatorApprovals[owner][operator];
    }

    function delegateAccess(address assistant) public virtual {
        require(assistant != address(0), "Zero address");
        _getAgentStorage().accessAssistants[msg.sender] = assistant;
        emit DelegateAccess(msg.sender, assistant);
    }

    function getDelegateAccess(address user) public view virtual returns (address) {
        return _getAgentStorage().accessAssistants[user];
    }

    function _isApprovedOrOwner(address spender, uint256 tokenId) internal view returns (bool) {
        require(_exists(tokenId), "Token does not exist");
        address owner = ownerOf(tokenId);
        return (spender == owner || getApproved(tokenId) == spender || isApprovedForAll(owner, spender));
    }

    function batchAuthorizeUsage(uint256 tokenId, address[] calldata users) public virtual {
        require(users.length > 0, "Empty users array");
        AgentNFTStorage storage $ = _getAgentStorage();
        require($.tokens[tokenId].owner == msg.sender, "Not owner");

        address[] storage authorizedUsers = $.tokens[tokenId].authorizedUsers;

        require(authorizedUsers.length + users.length <= MAX_AUTHORIZED_USERS, "Too many authorized users");

        for (uint256 i = 0; i < users.length; i++) {
            require(users[i] != address(0), "Zero address in users");

            bool alreadyAuthorized = false;
            for (uint256 j = 0; j < authorizedUsers.length; j++) {
                if (authorizedUsers[j] == users[i]) {
                    alreadyAuthorized = true;
                    break;
                }
            }
            require(!alreadyAuthorized, "User already authorized");

            for (uint256 k = 0; k < i; k++) {
                require(users[k] != users[i], "Duplicate user in batch");
            }

            authorizedUsers.push(users[i]);
            emit Authorization(msg.sender, users[i], tokenId);
        }
    }

    function revokeAuthorization(uint256 tokenId, address user) public virtual {
        AgentNFTStorage storage $ = _getAgentStorage();
        require($.tokens[tokenId].owner == msg.sender, "Not owner");
        require(user != address(0), "Zero address");

        address[] storage authorizedUsers = $.tokens[tokenId].authorizedUsers;
        bool found = false;

        for (uint256 i = 0; i < authorizedUsers.length; i++) {
            if (authorizedUsers[i] == user) {
                authorizedUsers[i] = authorizedUsers[authorizedUsers.length - 1];
                authorizedUsers.pop();
                found = true;
                break;
            }
        }

        require(found, "User not authorized");
        emit AuthorizationRevoked(msg.sender, user, tokenId);
    }

    function clearAuthorizedUsers(uint256 tokenId) public virtual {
        AgentNFTStorage storage $ = _getAgentStorage();
        require($.tokens[tokenId].owner == msg.sender, "Not owner");

        delete $.tokens[tokenId].authorizedUsers;
        emit AuthorizedUsersCleared(msg.sender, tokenId);
    }

    event AuthorizedUsersCleared(address indexed owner, uint256 indexed tokenId);
}
