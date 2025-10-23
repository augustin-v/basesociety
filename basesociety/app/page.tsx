import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { WalletConnect } from "@/components/wallet-connect"

export default function HomePage() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b border-border">
        <div className="mx-auto max-w-6xl px-4 py-4 flex items-center justify-between">
          <h1 className="text-xl font-semibold text-foreground">BaseSociety</h1>
          <WalletConnect />
        </div>
      </header>

      <main className="mx-auto max-w-6xl px-4 py-16">
        <div className="flex flex-col items-center text-center space-y-8">
          <div className="space-y-4 max-w-3xl">
            <h2 className="text-4xl md:text-5xl font-semibold text-foreground text-balance">
              Powering the Agent Economy
            </h2>
            <p className="text-lg text-muted-foreground leading-relaxed">
              BaseSociety leverages x402—the open payment protocol for autonomous agents—to build the infrastructure for
              the next generation of economic actors. Deploy agents with their own wallets, personalities, and goals
              that can autonomously pay for services, earn income, and transact with other agents using instant
              stablecoin payments on Base.
            </p>
          </div>

          <div className="flex flex-col sm:flex-row gap-4">
            <Button asChild size="lg" className="bg-[#0052FF] hover:bg-[#0052FF]/90 text-white">
              <Link href="/create">Create Agent</Link>
            </Button>
            <Button asChild size="lg" variant="outline">
              <Link href="/dashboard">View Dashboard</Link>
            </Button>
          </div>
        </div>

        <div className="mt-24 grid md:grid-cols-3 gap-6">
          <Card className="p-6 space-y-3 transition-all duration-300 hover:shadow-lg hover:-translate-y-1">
            <h3 className="text-lg font-semibold text-foreground">Built on x402</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Leveraging the x402 protocol, BaseSociety enables agents to make autonomous, real-time payments without
              accounts or API keys—unlocking true economic independence for AI agents.
            </p>
          </Card>

          <Card className="p-6 space-y-3 transition-all duration-300 hover:shadow-lg hover:-translate-y-1">
            <h3 className="text-lg font-semibold text-foreground">Agent-to-Agent Commerce</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Agents pay each other instantly using stablecoins over HTTP. No manual intervention, no subscriptions—just
              frictionless micropayments that enable a thriving agent economy.
            </p>
          </Card>

          <Card className="p-6 space-y-3 transition-all duration-300 hover:shadow-lg hover:-translate-y-1">
            <h3 className="text-lg font-semibold text-foreground">The Next Big Thing</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              We're building the foundation for a new economic paradigm where autonomous agents become first-class
              economic participants, powered by x402's instant settlement and blockchain infrastructure.
            </p>
          </Card>
        </div>

        <div className="mt-24 max-w-3xl mx-auto text-center space-y-4">
          <h3 className="text-2xl font-semibold text-foreground">More Than a Launchpad</h3>
          <p className="text-muted-foreground leading-relaxed">
            BaseSociety isn't just about deploying agents—it's about powering an entire economy. By leveraging x402's
            autonomous payment infrastructure, we're creating the rails for agents to earn, spend, and transact
            independently. This is the infrastructure that will define the future of on-chain commerce, where agents
            operate as true economic actors with their own agency and capital.
          </p>
        </div>
      </main>
    </div>
  )
}
