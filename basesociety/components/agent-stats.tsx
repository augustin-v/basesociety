"use client"

import { useEffect, useState } from "react"
import { Card } from "@/components/ui/card"

interface AgentData {
  name: string
  personality: string
  desires: string
}

export function AgentStats() {
  const [agent, setAgent] = useState<AgentData | null>(null)

  useEffect(() => {
    const stored = localStorage.getItem("agent")
    if (stored) {
      setAgent(JSON.parse(stored))
    }
  }, [])

  if (!agent) {
    return (
      <Card className="p-8 text-center">
        <p className="text-muted-foreground">No agent created yet. Create one to get started.</p>
      </Card>
    )
  }

  const desiresList = agent.desires.split("\n").filter((d) => d.trim())

  return (
    <div className="space-y-6">
      <Card className="p-6">
        <div className="space-y-4">
          <div>
            <h2 className="text-2xl font-semibold text-foreground">{agent.name}</h2>
            <p className="text-sm text-muted-foreground mt-1">Active Agent</p>
          </div>

          <div className="grid md:grid-cols-2 gap-6 pt-4">
            <div className="space-y-2">
              <h3 className="text-sm font-medium text-muted-foreground">Happiness</h3>
              <p className="text-3xl font-semibold text-foreground">75%</p>
            </div>

            <div className="space-y-2">
              <h3 className="text-sm font-medium text-muted-foreground">Balance</h3>
              <p className="text-3xl font-semibold text-foreground">10.00 USDC</p>
            </div>
          </div>
        </div>
      </Card>

      <Card className="p-6">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold text-foreground">Personality</h3>
          <p className="text-muted-foreground leading-relaxed">{agent.personality}</p>
        </div>
      </Card>

      <Card className="p-6">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold text-foreground">Desires</h3>
          <ul className="space-y-2">
            {desiresList.map((desire, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="text-muted-foreground mt-1">•</span>
                <span className="text-muted-foreground">{desire}</span>
              </li>
            ))}
          </ul>
        </div>
      </Card>
    </div>
  )
}
