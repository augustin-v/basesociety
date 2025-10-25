"use client"

import type React from "react"
import { useState } from "react"
import { useRouter } from "next/navigation"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { useAccount } from "@/lib/wallet-context"
import { CONTRACT_ADDRESS, API_BASE } from "@/lib/config"
import { AgentNFTABI } from "@/lib/agent-nft-abi"
import { createPublicClient, createWalletClient, custom, http, keccak256, toHex, encodeFunctionData } from "viem"
import { baseSepolia } from "viem/chains"

export function CreateAgentForm() {
  const router = useRouter()
  const { address } = useAccount()
  const [formData, setFormData] = useState({
    name: "",
    personality: "",
    desires: "",
    skills: "",
  })
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)
    setError(null)
  
    if (!address) {
      setError("Please connect your wallet first")
      setIsSubmitting(false)
      return
    }
  
    try {
      const publicClient = createPublicClient({
        chain: baseSepolia,
        transport: http(),
      })
  
      const hasEthereum = typeof window !== "undefined" && window.ethereum
  
      let hash: `0x${string}`
      let tokenId: bigint
  
      // Split skills into array *once* here (reuse for both contract and API)
      const skillsArray = formData.skills
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean)
  
      if (hasEthereum) {
        const walletClient = createWalletClient({
          chain: baseSepolia,
          transport: custom(window.ethereum),
        })
  
        const data = encodeFunctionData({
          abi: AgentNFTABI,
          functionName: "mint",
          args: [
            [
              {
                dataDescription: "Agent creation data",
                dataHash: keccak256(
                  toHex(JSON.stringify({ personality: formData.personality, desires: formData.desires })),
                ),
              },
            ],
            address,
            {
              personality: formData.personality,
              desires: formData.desires,
              skills: skillsArray,  // Already using the array here
              activityLogHash: "0x0000000000000000000000000000000000000000000000000000000000000000" as `0x${string}`,
              lastPassionTimestamp: 0n,
              happinessScore: 80,
            },
          ],
        })
  
        console.log("[v0] Encoded data:", data)
  
        hash = await walletClient.sendTransaction({
          to: CONTRACT_ADDRESS,
          data,
          account: address,
        })
  
        console.log("[v0] Transaction hash:", hash)
  
        const receipt = await publicClient.waitForTransactionReceipt({ hash })
        console.log("[v0] Transaction status:", receipt.status)
  
        if (receipt.status === "reverted") {
          throw new Error("Transaction reverted. Please check your contract and try again.")
        }
  
        const mintedEventTopic = keccak256(toHex("Minted(uint256,address,address)"))
        const mintedLog = receipt.logs.find((log) => log.topics[0] === mintedEventTopic)
  
        console.log("[v0] Minted log:", mintedLog)
  
        if (!mintedLog || !mintedLog.topics[1]) {
          throw new Error("Failed to parse tokenId from transaction receipt")
        }
  
        tokenId = BigInt(mintedLog.topics[1])
        console.log("[v0] Parsed tokenId:", tokenId.toString())
      } else {
        console.log("[v0] Demo mode: simulating contract interaction")
        await new Promise((resolve) => setTimeout(resolve, 2000))
        hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" as `0x${string}`
        tokenId = BigInt(Math.floor(Math.random() * 1000000))
      }
  
      const agentId = `agent-${Date.now()}`
      const profileData = {
        name: formData.name,
        personality: formData.personality,
        desires: formData.desires,
        skills: skillsArray,  // FIX: Send the array, not the raw string!
      }
  
      const response = await fetch(`${API_BASE}/agents`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          agent_id: agentId,
          owner_address: address,
          token_id: tokenId.toString(),
          profile: profileData,  // Now includes skills as ["following", "orders", ...]
        }),
      })
  
      if (!response.ok) {
        console.warn("[v0] API registration failed, using local storage")
        localStorage.setItem(
          "agent",
          JSON.stringify({
            ...profileData,
            tokenId: tokenId.toString(),
            transactionHash: hash,
            // Include skillsArray here too if needed for local storage
            skills: skillsArray,
          }),
        )
      } else {
        // NEW: Parse the agent ID from response body (backend sends it back)
        const createdAgentId = await response.json();  // String like "agent-1761313683392"
        localStorage.setItem("agentId", createdAgentId);
        console.log("[v0] Agent created & ID stored:", createdAgentId);
      }
  
      router.push("/dashboard")
    } catch (err) {
      console.error("[v0] Error creating agent:", err)
      setError(err instanceof Error ? err.message : "Failed to create agent. Please try again.")
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Card className="p-6">
      <form onSubmit={handleSubmit} className="space-y-6">
        {error && (
          <div className="rounded-lg bg-red-50 p-4 text-sm text-red-800 border border-red-200">
            <strong>Error:</strong> {error}
          </div>
        )}

        {!address && (
          <div className="rounded-lg bg-yellow-50 p-4 text-sm text-yellow-800 border border-yellow-200">
            Please connect your wallet to create an agent.
          </div>
        )}

        <div className="space-y-2">
          <Label htmlFor="name">Agent Name</Label>
          <Input
            id="name"
            placeholder="Enter agent name"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="personality">Personality</Label>
          <Textarea
            id="personality"
            placeholder="Describe your agent's personality traits"
            value={formData.personality}
            onChange={(e) => setFormData({ ...formData, personality: e.target.value })}
            rows={4}
            required
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="desires">Desires</Label>
          <Textarea
            id="desires"
            placeholder="List your agent's goals and desires"
            value={formData.desires}
            onChange={(e) => setFormData({ ...formData, desires: e.target.value })}
            rows={4}
            required
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="skills">Skills (comma-separated)</Label>
          <Input
            id="skills"
            placeholder="e.g., trading, analysis, communication"
            value={formData.skills}
            onChange={(e) => setFormData({ ...formData, skills: e.target.value })}
          />
        </div>

        <div className="flex gap-4">
          <Button
            type="submit"
            disabled={isSubmitting || !address}
            className="bg-[#0052FF] hover:bg-[#0052FF]/90 text-white"
          >
            {isSubmitting ? "Creating..." : "Create Agent"}
          </Button>
          <Button type="button" variant="outline" onClick={() => router.push("/")}>
            Cancel
          </Button>
        </div>
      </form>
    </Card>
  )
}
