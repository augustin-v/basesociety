"use client"

import type React from "react"
// import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
// import { WagmiProvider } from "wagmi"
// import { config } from "@/lib/wagmi-config"
import { WalletProvider } from "@/lib/wallet-context"

export function Providers({ children }: { children: React.ReactNode }) {
  return <WalletProvider>{children}</WalletProvider>
}
