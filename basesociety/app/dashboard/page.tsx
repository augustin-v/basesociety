import Link from "next/link"
import { WalletConnect } from "@/components/wallet-connect"
import { AgentStats } from "@/components/agent-stats"

export default function DashboardPage() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b border-border">
        <div className="mx-auto max-w-6xl px-4 py-4 flex items-center justify-between">
          <Link href="/" className="text-xl font-semibold text-foreground hover:text-foreground/80">
            BaseSociety
          </Link>
          <WalletConnect />
        </div>
      </header>

      <main className="mx-auto max-w-6xl px-4 py-16">
        <div className="space-y-8">
          <div className="flex items-center justify-between">
            <div className="space-y-1">
              <h1 className="text-3xl font-semibold text-foreground">Agent Dashboard</h1>
              <p className="text-muted-foreground">
                Monitor your agent's economic activity and performance in the agent economy.
              </p>
            </div>
          </div>

          <AgentStats />
        </div>
      </main>
    </div>
  )
}
