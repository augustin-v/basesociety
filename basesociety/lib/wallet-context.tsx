"use client"

import type React from "react"
import { createContext, useContext, useState, useCallback, useEffect } from "react"

interface WalletContextType {
  address: `0x${string}` | undefined
  isConnected: boolean
  connect: () => Promise<void>
  disconnect: () => void
}

const WalletContext = createContext<WalletContextType | undefined>(undefined)

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [address, setAddress] = useState<`0x${string}` | undefined>(undefined)
  const [isConnected, setIsConnected] = useState(false)

  useEffect(() => {
    const checkConnection = async () => {
      if (typeof window !== "undefined" && window.ethereum) {
        try {
          const accounts = await window.ethereum.request({ method: "eth_accounts" })
          if (accounts && accounts.length > 0) {
            setAddress(accounts[0] as `0x${string}`)
            setIsConnected(true)
          }
        } catch (error) {
          console.error("Failed to check wallet connection:", error)
        }
      }
    }
    checkConnection()
  }, [])

  useEffect(() => {
    if (typeof window !== "undefined" && window.ethereum) {
      const handleAccountsChanged = (accounts: string[]) => {
        if (accounts.length > 0) {
          setAddress(accounts[0] as `0x${string}`)
          setIsConnected(true)
        } else {
          setAddress(undefined)
          setIsConnected(false)
        }
      }

      window.ethereum.on("accountsChanged", handleAccountsChanged)

      return () => {
        window.ethereum?.removeListener("accountsChanged", handleAccountsChanged)
      }
    }
  }, [])

  const connect = useCallback(async () => {
    try {
      if (typeof window === "undefined" || !window.ethereum) {
        throw new Error("Please install MetaMask or another Web3 wallet")
      }

      const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
      })

      if (accounts && accounts.length > 0) {
        setAddress(accounts[0] as `0x${string}`)
        setIsConnected(true)
      }
    } catch (error) {
      console.error("Failed to connect wallet:", error)
      throw error
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

export function useAccount() {
  const context = useContext(WalletContext)
  if (context === undefined) {
    throw new Error("useAccount must be used within a WalletProvider")
  }
  return { address: context.address, isConnected: context.isConnected }
}

export function useWallet() {
  const context = useContext(WalletContext)
  if (context === undefined) {
    throw new Error("useWallet must be used within a WalletProvider")
  }
  return context
}
