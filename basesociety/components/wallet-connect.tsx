"use client"

import { useWallet } from "@/lib/wallet-context"
import { Button } from "@/components/ui/button"

export function WalletConnect() {
  const { address, isConnected, connect, disconnect } = useWallet()

  if (isConnected && address) {
    return (
      <Button variant="outline" onClick={disconnect} className="font-mono text-sm bg-transparent">
        {address.slice(0, 6)}...{address.slice(-4)}
      </Button>
    )
  }

  return (
    <Button onClick={connect} className="bg-[#0052FF] hover:bg-[#0052FF]/90 text-white">
      Connect Wallet
    </Button>
  )
}
