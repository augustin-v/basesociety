import { http, createConfig } from "wagmi"
import { baseSepolia } from "wagmi/chains"
import { injected } from "wagmi/connectors"

export const config = createConfig({
  chains: [baseSepolia],
  connectors: [
    injected({
      shimDisconnect: true,
    }),
  ],
  transports: {
    [baseSepolia.id]: http(),
  },
  ssr: true,
})
