"use client"

import { useEffect, useState } from "react"
import { useAccount } from "@/lib/wallet-context"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area"
import { API_BASE } from "@/lib/config"
import type { CustomMessage } from "@/lib/types"

interface AgentHistoryProps {
  agentId: string
}

export function AgentHistory({ agentId }: AgentHistoryProps) {
  const { address } = useAccount()
  const [history, setHistory] = useState<CustomMessage[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!address) return

    const fetchHistory = async () => {
      try {
        const response = await fetch(`${API_BASE}/agents/${agentId}/history`, {
          headers: {
            "Content-Type": "application/json",
            "X-Owner-Address": address,
          },
        })

        if (!response.ok) {
          if (response.status === 403) throw new Error("Access denied: Only owner can view history")
          throw new Error(`Failed to fetch history: ${response.statusText}`)
        }

        const data: CustomMessage[] = await response.json()
        setHistory(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : "Unknown error")
      } finally {
        setLoading(false)
      }
    }

    fetchHistory()
  }, [agentId, address])

  if (loading) return <Card className="p-4"><p>Loading history...</p></Card>
  if (error) return <Card className="p-4 text-destructive">{error}</Card>

  return (
    <Card className="p-6">
      <CardHeader>
        <CardTitle>Thoughts History</CardTitle>
      </CardHeader>
      <CardContent>
        {history.length === 0 ? (
          <p className="text-muted-foreground">No history yet. Interact with your agent to start!</p>
        ) : (
          <ScrollArea className="h-64 rounded-md border p-2">
            <div className="space-y-2">
              {history.map((msg, idx) => (
                <div key={idx} className={`p-2 rounded ${msg.origin === 'Owner' ? 'bg-blue-50 ml-auto max-w-xs' : 'bg-gray-50 max-w-xs'}`}>
                  <div className="text-xs text-muted-foreground mb-1">
                    {new Date(msg.timestamp).toLocaleString()} - {msg.origin}
                  </div>
                  <div className="text-sm">{msg.content}</div>
                </div>
              ))}
            </div>
          </ScrollArea>
        )}
      </CardContent>
    </Card>
  )
}