"use client"

import type React from "react"
import { createContext, useContext, useState, useCallback } from "react"

interface WalletContextType {
  address: string | undefined
  isConnected: boolean
  connect: () => Promise<void>
  disconnect: () => void
}

const WalletContext = createContext<WalletContextType | undefined>(undefined)

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [address, setAddress] = useState<string | undefined>(undefined)
  const [isConnected, setIsConnected] = useState(false)

  const connect = useCallback(async () => {
    // In production, this would use wagmi/viem to connect to actual wallets
    try {
      // Simulate wallet connection
      const mockAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
      setAddress(mockAddress)
      setIsConnected(true)
    } catch (error) {
      console.error("Failed to connect wallet:", error)
    }
  }, [])

  const disconnect = useCallback(() => {
    setAddress(undefined)
    setIsConnected(false)
  }, [])

  return (
    <WalletContext.Provider value={{ address, isConnected, connect, disconnect }}>{children}</WalletContext.Provider>
  )
}

export function useWallet() {
  const context = useContext(WalletContext)
  if (context === undefined) {
    throw new Error("useWallet must be used within a WalletProvider")
  }
  return context
}
