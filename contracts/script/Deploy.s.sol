// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {AgentNFT} from "../src/AgentsNFT.sol";
import {DecayOracle} from "../src/DecayOracle.sol";
import {MockDataVerifier} from "../src/MockDataVerifier.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployScript is Script {
    AgentNFT public agentNFT;
    DecayOracle public decayOracle;
    MockDataVerifier public mockDataVerifier;

    function run() public {
        uint256 deployerPrivateKey = uint256(vm.envBytes32("PRIVATE_KEY"));
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        // 1. Deploy MockDataVerifier
        mockDataVerifier = new MockDataVerifier();
        console.log("MockDataVerifier deployed at:", address(mockDataVerifier));

        // 2. Deploy AgentNFT
        AgentNFT agentNFTImplementation = new AgentNFT();
        bytes memory agentNFTData = abi.encodeWithSelector(
            AgentNFT.initialize.selector,
            "Agents NFT",
            "AGENT",
            address(mockDataVerifier),
            deployerAddress
        );
        ERC1967Proxy agentNFTProxy = new ERC1967Proxy(address(agentNFTImplementation), agentNFTData);
        agentNFT = AgentNFT(address(agentNFTProxy));
        console.log("AgentNFT deployed at:", address(agentNFT));

        // 3. Deploy DecayOracle
        DecayOracle decayOracleImplementation = new DecayOracle();
        bytes memory decayOracleData = abi.encodeWithSelector(
            DecayOracle.initialize.selector,
            deployerAddress,
            address(agentNFT)
        );
        ERC1967Proxy decayOracleProxy = new ERC1967Proxy(address(decayOracleImplementation), decayOracleData);
        decayOracle = DecayOracle(address(decayOracleProxy));
        console.log("DecayOracle deployed at:", address(decayOracle));

        // 4. Set the oracle on AgentNFT
        agentNFT.setOracle(address(decayOracle));
        console.log("Oracle set on AgentsNFT");

        vm.stopBroadcast();
    }
}
