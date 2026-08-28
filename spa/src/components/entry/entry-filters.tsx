import { X } from "lucide-react"
import { Button } from "@/components/ui/button"
import { MOODS } from "@/lib/mood"
import { cn } from "@/lib/utils"
import { useActivities } from "@/hooks/use-activities"
import type { EntryFilters } from "@/hooks/use-entries"

interface EntryFiltersBarProps {
  filters: EntryFilters
  onChange: (filters: EntryFilters) => void
}

export function isNarrowed(filters: EntryFilters): boolean {
  return Object.values(filters).some((value) => value !== undefined)
}

export function EntryFiltersBar({ filters, onChange }: EntryFiltersBarProps) {
  const { data: activities } = useActivities()
  const inUse = activities?.filter((activity) => !activity.archived) ?? []

  const toggleMood = (mood: number) =>
    onChange({ ...filters, mood: filters.mood === mood ? undefined : mood })

  const chooseActivity = (activity: string) =>
    onChange({
      ...filters,
      activity: filters.activity === activity ? undefined : activity,
    })

  return (
    <div className="flex flex-col gap-3">
      <fieldset className="flex flex-col gap-1.5">
        <legend className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
          Mood
        </legend>
        <div className="flex gap-1.5">
          {MOODS.map((mood) => (
            <button
              key={mood.value}
              type="button"
              aria-pressed={filters.mood === mood.value}
              aria-label={mood.label}
              onClick={() => toggleMood(mood.value)}
              className={cn(
                "flex-1 rounded-lg border py-1.5 text-xl transition-all",
                filters.mood === mood.value
                  ? "border-primary bg-primary/15"
                  : "border-transparent opacity-60 hover:opacity-100"
              )}
            >
              <span aria-hidden="true">{mood.emoji}</span>
            </button>
          ))}
        </div>
      </fieldset>

      {inUse.length > 0 && (
        <fieldset className="flex flex-col gap-1.5">
          <legend className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            Activity
          </legend>
          <div className="flex flex-wrap gap-1.5">
            {inUse.map((activity) => (
              <button
                key={activity.id}
                type="button"
                aria-pressed={filters.activity === activity.id}
                onClick={() => chooseActivity(activity.id)}
                className={cn(
                  "rounded-full border px-2.5 py-1 text-xs transition-colors",
                  filters.activity === activity.id
                    ? "border-primary bg-primary/15 text-foreground"
                    : "border-border text-muted-foreground hover:text-foreground"
                )}
              >
                {activity.name}
              </button>
            ))}
          </div>
        </fieldset>
      )}

      {isNarrowed(filters) && (
        <Button
          variant="ghost"
          size="sm"
          className="self-start"
          onClick={() => onChange({})}
        >
          <X className="mr-1 h-3 w-3" aria-hidden="true" />
          Clear filters
        </Button>
      )}
    </div>
  )
}
