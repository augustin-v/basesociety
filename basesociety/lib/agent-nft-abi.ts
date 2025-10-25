export const AgentNFTABI = [
  {
    type: "function",
    name: "mint",
    inputs: [
      {
        name: "iDatas",
        type: "tuple[]",
        internalType: "struct IntelligentData[]",
        components: [
          { name: "dataDescription", type: "string", internalType: "string" },
          { name: "dataHash", type: "bytes32", internalType: "bytes32" },
        ],
      },
      {
        name: "to",
        type: "address",
        internalType: "address",
      },
      {
        name: "profile",
        type: "tuple",
        internalType: "struct AgentProfile",
        components: [
          { name: "personality", type: "string", internalType: "string" },
          { name: "desires", type: "string", internalType: "string" },
          { name: "skills", type: "string[]", internalType: "string[]" },
          { name: "activityLogHash", type: "bytes32", internalType: "bytes32" },
          { name: "lastPassionTimestamp", type: "uint256", internalType: "uint256" },
          { name: "happinessScore", type: "uint8", internalType: "uint8" },
        ],
      },
    ],
    outputs: [
      {
        name: "tokenId",
        type: "uint256",
        internalType: "uint256",
      },
    ],
    stateMutability: "payable",
  },
  {
    type: "event",
    name: "Minted",
    inputs: [
      {
        name: "tokenId",
        type: "uint256",
        indexed: true,
      },
      {
        name: "minter",
        type: "address",
        indexed: true,
      },
      {
        name: "to",
        type: "address",
        indexed: false,
      },
    ],
  },
] as const

export type AgentProfile = {
  personality: string
  desires: string
  skills: string[]
  activityLogHash: `0x${string}`
  lastPassionTimestamp: bigint
  happinessScore: number
}
