import Link from "next/link"
import { CreateAgentForm } from "@/components/create-agent-form"
import { WalletConnect } from "@/components/wallet-connect"

export default function CreateAgentPage() {
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

      <main className="mx-auto max-w-2xl px-4 py-16">
        <div className="space-y-6">
          <div className="space-y-2">
            <h1 className="text-3xl font-semibold text-foreground">Create Agent</h1>
            <p className="text-muted-foreground">
              Deploy a new autonomous economic actor with its own personality, goals, and wallet. Join the agent
              economy.
            </p>
          </div>

          <CreateAgentForm />
        </div>
      </main>
    </div>
  )
}
