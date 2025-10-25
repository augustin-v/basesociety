"use client"

import { useEffect, useState } from "react"
import { useAccount } from "@/lib/wallet-context"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { API_BASE } from "@/lib/config"
import type { AgentDetails, CustomMessage } from "@/lib/types"  // Adjust path
import { AgentHistory } from "./agent-history"

export function AgentStats({ agentId }: { agentId?: string }) {
  const { address } = useAccount()
  const [agent, setAgent] = useState<AgentDetails | null>(null)
  const [lastThought, setLastThought] = useState<string | null>(null)
  const [showHistory, setShowHistory] = useState(false)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Fallback to localStorage if no prop (for now)
  const id = agentId || localStorage.getItem("agentId")

  useEffect(() => {
    if (!address || !id) {
      setLoading(false)
      if (!address) setError("Connect wallet to view agent stats")
      else setError("No agent ID found. Create one first.")
      return
    }

    const fetchData = async () => {
      try {
        // Fetch agent details (name, profile)
        const detailsRes = await fetch(`${API_BASE}/agents/${id}`, {
          headers: {
            "Content-Type": "application/json",
            "X-Owner-Address": address,
          },
        })
        if (!detailsRes.ok) {
          if (detailsRes.status === 403) throw new Error("Access denied: Not your agent")
          throw new Error(`Failed to fetch agent: ${detailsRes.statusText}`)
        }
        const details: AgentDetails = await detailsRes.json()
        setAgent(details)

        // Fetch history for last thought
        const historyRes = await fetch(`${API_BASE}/agents/${id}/history`, {
          headers: {
            "Content-Type": "application/json",
            "X-Owner-Address": address,
          },
        })
        if (historyRes.ok) {
          const history: CustomMessage[] = await historyRes.json()
          // Latest agent thought (sort DESC by timestamp)
          const recentThoughts = history
            .filter(msg => msg.origin === 'Agent')
            .sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
          setLastThought(recentThoughts[0]?.content || null)
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to load stats")
      } finally {
        setLoading(false)
      }
    }

    fetchData()
  }, [id, address])

  if (loading) return <Card className="p-8 text-center"><p>Loading agent stats...</p></Card>
  if (error || !agent) return (
    <Card className="p-8 text-center">
      <p className="text-destructive">{error || "No agent selected. Create or select one."}</p>
    </Card>
  )

  const desiresList = agent.profile.desires.split("\n").filter(d => d.trim())

  return (
    <div className="space-y-6">
      <Card className="p-6">
        <CardHeader>
          <div>
            <h2 className="text-2xl font-semibold text-foreground">{agent.profile.name}</h2>
            <p className="text-sm text-muted-foreground mt-1">Active Agent (ID: {agent.agent_id})</p>
          </div>
        </CardHeader>
        <CardContent>
          <div className="grid md:grid-cols-2 gap-6 pt-4">
            <div className="space-y-2">
              <h3 className="text-sm font-medium text-muted-foreground">Happiness</h3>
              <p className="text-3xl font-semibold text-foreground">75%</p>  {/* TODO: Fetch from SC/DB */}
            </div>
            <div className="space-y-2">
              <h3 className="text-sm font-medium text-muted-foreground">Balance</h3>
              <p className="text-3xl font-semibold text-foreground">10.00 USDC</p>  {/* TODO: Fetch from SC */}
            </div>
          </div>
          {/* NEW: Last Thought Teaser */}
          {lastThought && (
            <div className="pt-4 border-t mt-4">
              <h3 className="text-sm font-medium text-muted-foreground mb-2">Latest Thought</h3>
              <Badge variant="secondary" className="mb-2">Owner Only</Badge>
              <p className="text-sm text-foreground italic leading-relaxed">{lastThought.substring(0, 150)}...</p>
            </div>
          )}
          {/* NEW: History Toggle */}
          <Button onClick={() => setShowHistory(!showHistory)} className="mt-4">
            {showHistory ? "Hide" : "View"} Thoughts History
          </Button>
        </CardContent>
      </Card>

      <Card className="p-6">
        <CardHeader><CardTitle>Personality</CardTitle></CardHeader>
        <CardContent><p className="text-muted-foreground leading-relaxed">{agent.profile.personality}</p></CardContent>
      </Card>

      <Card className="p-6">
        <CardHeader><CardTitle>Desires</CardTitle></CardHeader>
        <CardContent>
          <ul className="space-y-2">
            {desiresList.map((desire, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="text-muted-foreground mt-1">•</span>
                <span className="text-muted-foreground">{desire}</span>
              </li>
            ))}
          </ul>
        </CardContent>
      </Card>

      {/* NEW: Full History (Toggled) */}
      {showHistory && <AgentHistory agentId={agent.agent_id} />}
    </div>
  )
}