"use client"

import { useWallet } from "@/lib/wallet-context"
import { Button } from "@/components/ui/button"
import { useState } from "react"

export function WalletConnect() {
  const { address, isConnected, connect, disconnect } = useWallet()
  const [error, setError] = useState<string | null>(null)

  const handleConnect = async () => {
    try {
      setError(null)
      await connect()
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Failed to connect wallet"
      setError(errorMessage)
      alert(errorMessage)
    }
  }

  if (isConnected && address) {
    return (
      <Button variant="outline" onClick={disconnect} className="font-mono text-sm bg-transparent">
        {address.slice(0, 6)}...{address.slice(-4)}
      </Button>
    )
  }

  return (
    <Button onClick={handleConnect} className="bg-[#0052FF] hover:bg-[#0052FF]/90 text-white">
      Connect Wallet
    </Button>
  )
}
