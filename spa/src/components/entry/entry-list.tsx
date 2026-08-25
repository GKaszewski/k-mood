import { format, isToday, isYesterday } from "date-fns"
import type { EntryResponse } from "@/api/schema"
import { EntryCard } from "./entry-card"

interface EntryListProps {
  entries: EntryResponse[]
  limit?: number
}

export function EntryList({ entries, limit }: EntryListProps) {
  const sliced = limit ? entries.slice(0, limit) : entries
  const grouped = groupByDay(sliced)

  return (
    <div className="flex flex-col gap-4">
      {grouped.map(({ label, entries }) => (
        <div key={label} className="flex flex-col gap-2">
          <h3 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {label}
          </h3>
          {entries.map((entry) => (
            <EntryCard key={entry.id} entry={entry} />
          ))}
        </div>
      ))}
    </div>
  )
}

function groupByDay(
  entries: EntryResponse[]
): Array<{ label: string; entries: EntryResponse[] }> {
  const groups = new Map<string, EntryResponse[]>()

  for (const entry of entries) {
    const date = new Date(entry.loggedAt)
    const key = format(date, "yyyy-MM-dd")

    if (!groups.has(key)) {
      groups.set(key, [])
    }
    groups.get(key)!.push(entry)
  }

  return Array.from(groups.entries()).map(([key, entries]) => ({
    label: formatDayLabel(new Date(key)),
    entries,
  }))
}

function formatDayLabel(date: Date): string {
  if (isToday(date)) return "Today"
  if (isYesterday(date)) return "Yesterday"
  return format(date, "EEEE, MMMM d, yyyy")
}
