import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"
import type { ActivityResponse } from "@/api/schema"

interface ActivityPickerProps {
  activities: ActivityResponse[]
  selected: string[]
  onToggle: (id: string) => void
}

export function ActivityPicker({
  activities,
  selected,
  onToggle,
}: ActivityPickerProps) {
  const grouped = groupByCategory(activities)

  return (
    <div className="flex flex-col gap-3">
      {grouped.map(({ category, items }) => (
        <div key={category}>
          <p className="mb-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {category}
          </p>
          <div className="flex flex-wrap gap-2">
            {items.map((activity) => (
              <Badge
                key={activity.id}
                variant={selected.includes(activity.id) ? "default" : "outline"}
                className={cn(
                  "cursor-pointer transition-colors",
                  selected.includes(activity.id) && "bg-primary"
                )}
                onClick={() => onToggle(activity.id)}
              >
                {activity.name}
              </Badge>
            ))}
          </div>
        </div>
      ))}
    </div>
  )
}

function groupByCategory(
  activities: ActivityResponse[]
): Array<{ category: string; items: ActivityResponse[] }> {
  const map = new Map<string, ActivityResponse[]>()

  for (const a of activities) {
    const cat = a.category ?? "Other"
    if (!map.has(cat)) map.set(cat, [])
    map.get(cat)!.push(a)
  }

  return Array.from(map.entries())
    .sort(([a], [b]) => {
      if (a === "Other") return 1
      if (b === "Other") return -1
      return a.localeCompare(b)
    })
    .map(([category, items]) => ({ category, items }))
}
