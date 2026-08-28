import { describeDay } from "@/lib/day-label"
import { formatDate, parseDate } from "@/lib/day"
import type { EntryResponse } from "@/api/schema"
import { EntryCard } from "./entry-card"

interface EntryListProps {
  entries: EntryResponse[]
}

export function EntryList({ entries }: EntryListProps) {
  return (
    <div className="flex flex-col gap-4">
      {groupByDay(entries).map(({ day, entries: onThatDay }) => (
        <section key={day} className="flex flex-col gap-2">
          <h3 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {describeDay(parseDate(day))}
          </h3>
          {onThatDay.map((entry) => (
            <EntryCard key={entry.id} entry={entry} />
          ))}
        </section>
      ))}
    </div>
  )
}

interface DayGroup {
  day: string
  entries: EntryResponse[]
}

function groupByDay(entries: EntryResponse[]): DayGroup[] {
  const groups = new Map<string, EntryResponse[]>()

  for (const entry of entries) {
    const day = formatDate(new Date(entry.loggedAt))
    const existing = groups.get(day)

    if (existing) {
      existing.push(entry)
    } else {
      groups.set(day, [entry])
    }
  }

  return Array.from(groups.entries()).map(([day, onThatDay]) => ({
    day,
    entries: onThatDay,
  }))
}
