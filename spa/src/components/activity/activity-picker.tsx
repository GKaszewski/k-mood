import { cn } from "@/lib/utils"
import { groupByCategory } from "@/lib/activity-groups"
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
  return (
    <div className="flex flex-col gap-3">
      {groupByCategory(activities).map(({ category, items }) => (
        <fieldset key={category}>
          <legend className="mb-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {category}
          </legend>
          <div className="flex flex-wrap gap-2">
            {items.map((activity) => {
              const isSelected = selected.includes(activity.id)

              return (
                <button
                  key={activity.id}
                  type="button"
                  role="checkbox"
                  aria-checked={isSelected}
                  onClick={() => onToggle(activity.id)}
                  className={cn(
                    "rounded-full border px-2.5 py-1 text-xs transition-colors",
                    "focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
                    isSelected
                      ? "border-primary bg-primary text-primary-foreground"
                      : "border-border text-muted-foreground hover:text-foreground"
                  )}
                >
                  {activity.name}
                </button>
              )
            })}
          </div>
        </fieldset>
      ))}
    </div>
  )
}
