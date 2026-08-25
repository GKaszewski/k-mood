import { useState } from "react"
import { createFileRoute } from "@tanstack/react-router"
import { Plus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"
import { useActivities, useCreateActivity } from "@/hooks/use-activities"
import { ActivityGroup } from "@/components/activity/activity-group"
import type { ActivityResponse } from "@/api/schema"

export const Route = createFileRoute("/_auth/activities")({
  component: ActivitiesPage,
})

function ActivitiesPage() {
  const { data: activities, isLoading } = useActivities()
  const createActivity = useCreateActivity()
  const [newName, setNewName] = useState("")
  const [newCategory, setNewCategory] = useState("")

  const grouped = groupByCategory(activities ?? [])

  const handleCreate = () => {
    if (!newName.trim()) return
    createActivity.mutate(
      { name: newName.trim(), category: newCategory.trim() || undefined },
      {
        onSuccess: () => {
          setNewName("")
        },
      }
    )
  }

  return (
    <div className="flex flex-col gap-4 py-6">
      <h1 className="text-xl font-bold">Activities</h1>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Add activity</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-2">
          <Input
            placeholder="Activity name"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
          />
          <Input
            placeholder="Category (optional)"
            value={newCategory}
            onChange={(e) => setNewCategory(e.target.value)}
          />
          <Button
            size="sm"
            onClick={handleCreate}
            disabled={!newName.trim() || createActivity.isPending}
          >
            <Plus className="mr-2 h-4 w-4" />
            {createActivity.isPending ? "Adding..." : "Add"}
          </Button>
        </CardContent>
      </Card>

      {isLoading ? (
        <Skeleton className="h-40" />
      ) : (
        grouped.map(({ category, items }) => (
          <ActivityGroup
            key={category}
            category={category}
            activities={items}
          />
        ))
      )}
    </div>
  )
}

function groupByCategory(
  activities: ActivityResponse[]
): Array<{ category: string; items: ActivityResponse[] }> {
  const map = new Map<string, ActivityResponse[]>()

  for (const a of activities) {
    const cat = a.category ?? "Uncategorized"
    if (!map.has(cat)) map.set(cat, [])
    map.get(cat)!.push(a)
  }

  return Array.from(map.entries())
    .sort(([a], [b]) => {
      if (a === "Uncategorized") return 1
      if (b === "Uncategorized") return -1
      return a.localeCompare(b)
    })
    .map(([category, items]) => ({ category, items }))
}
