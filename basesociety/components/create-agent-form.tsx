"use client"

import type React from "react"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"

export function CreateAgentForm() {
  const router = useRouter()
  const [formData, setFormData] = useState({
    name: "",
    personality: "",
    desires: "",
  })
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)

    // Simulate agent creation
    await new Promise((resolve) => setTimeout(resolve, 1000))

    // Store in localStorage for demo purposes
    localStorage.setItem("agent", JSON.stringify(formData))

    setIsSubmitting(false)
    router.push("/dashboard")
  }

  return (
    <Card className="p-6">
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="space-y-2">
          <Label htmlFor="name">Agent Name</Label>
          <Input
            id="name"
            placeholder="Enter agent name"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="personality">Personality</Label>
          <Textarea
            id="personality"
            placeholder="Describe your agent's personality traits"
            value={formData.personality}
            onChange={(e) => setFormData({ ...formData, personality: e.target.value })}
            rows={4}
            required
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="desires">Desires</Label>
          <Textarea
            id="desires"
            placeholder="List your agent's goals and desires (one per line)"
            value={formData.desires}
            onChange={(e) => setFormData({ ...formData, desires: e.target.value })}
            rows={4}
            required
          />
        </div>

        <div className="flex gap-4">
          <Button type="submit" disabled={isSubmitting} className="bg-[#0052FF] hover:bg-[#0052FF]/90 text-white">
            {isSubmitting ? "Creating..." : "Create Agent"}
          </Button>
          <Button type="button" variant="outline" onClick={() => router.push("/")}>
            Cancel
          </Button>
        </div>
      </form>
    </Card>
  )
}
