import { format } from "date-fns"
import { cn } from "@/lib/utils"
import { getMood, nearestMood } from "@/lib/mood"
import { weekStartsOn } from "@/lib/week"
import { formatDate } from "@/lib/day"
import { heatmapGrid } from "@/lib/heatmap-grid"
import type { CalendarDayResponse } from "@/api/schema"

const ROWS = [0, 1, 2, 3, 4, 5, 6]

interface MoodHeatmapGridProps {
  start: Date
  end: Date
  days: CalendarDayResponse[]
  /** What the screen-reader summary calls this span, e.g. "2026" or "Period A". */
  name: string
}

/**
 * The squares of a mood heatmap for any span of days, without a card around
 * them: the year view puts one in a card, Compare stacks two in one.
 */
export function MoodHeatmapGrid({
  start,
  end,
  days,
  name,
}: MoodHeatmapGridProps) {
  const moodMap = new Map<string, number>()
  for (const day of days) {
    if (day.mood) {
      moodMap.set(day.date, day.mood)
    }
  }

  const { weeks, grid } = heatmapGrid(start, end)
  const labels = monthLabels(grid, weeks)

  return (
    <>
      <p className="sr-only">{summarise(name, moodMap)}</p>
      <div aria-hidden="true" className="flex w-max gap-0.5">
        <div className="flex flex-col gap-0.5 pr-1 text-[9px] text-muted-foreground">
          {rowLabels().map((label, i) => (
            <div key={i} className="h-[11px] leading-[11px]">
              {label}
            </div>
          ))}
        </div>

        <div className="flex flex-col gap-0.5">
          <div className="mb-0.5 flex gap-0.5">
            {labels.map((label, weekIdx) => (
              <div
                key={weekIdx}
                className="w-[11px] text-center text-[8px] leading-[11px] whitespace-nowrap text-muted-foreground"
              >
                {label}
              </div>
            ))}
          </div>

          {ROWS.map((dayIdx) => (
            <div key={dayIdx} className="flex gap-0.5">
              {grid[dayIdx].map((day, weekIdx) => {
                if (!day) {
                  return <div key={weekIdx} className="h-[11px] w-[11px]" />
                }

                const mood = moodMap.get(formatDate(day))

                return (
                  <div
                    key={weekIdx}
                    className={cn(
                      "h-[11px] w-[11px] rounded-[2px]",
                      mood ? getMood(mood).background : "bg-muted/30"
                    )}
                    title={
                      mood
                        ? `${format(day, "MMM d")} — ${getMood(mood).label}`
                        : format(day, "MMM d")
                    }
                  />
                )
              })}
            </div>
          ))}
        </div>
      </div>
    </>
  )
}

export function MoodHeatmapLegend() {
  return (
    <div className="mt-3 flex items-center justify-end gap-1 text-[10px] text-muted-foreground">
      <span>Less</span>
      <div className="h-[11px] w-[11px] rounded-[2px] bg-muted/30" />
      {[1, 2, 3, 4, 5].map((m) => (
        <div
          key={m}
          className={cn(
            "h-[11px] w-[11px] rounded-[2px]",
            getMood(m).background
          )}
          title={getMood(m).label}
        />
      ))}
      <span>Better</span>
    </div>
  )
}

/**
 * A week column is named after the month its first day falls in, but only
 * when that month changes — so a span that starts mid-month is still labelled.
 */
function monthLabels(grid: (Date | null)[][], weeks: number): string[] {
  let previous: number | null = null

  return Array.from({ length: weeks }, (_, weekIdx) => {
    const firstDay = ROWS.map((row) => grid[row][weekIdx]).find(Boolean)
    if (!firstDay || firstDay.getMonth() === previous) return ""

    previous = firstDay.getMonth()
    return format(firstDay, "MMM")
  })
}

/**
 * The grid is hundreds of coloured squares — as tab stops it would be
 * unusable, and read out one by one it would be meaningless. This says the
 * same thing in a sentence; the calendar is where a screen reader goes for a
 * given day.
 */
function summarise(name: string, moods: Map<string, number>): string {
  const logged = moods.size

  if (logged === 0) return `Mood heatmap for ${name}. Nothing logged.`

  const total = Array.from(moods.values()).reduce((sum, mood) => sum + mood, 0)
  const average = nearestMood(total / logged)

  return `Mood heatmap for ${name}. ${logged} ${
    logged === 1 ? "day" : "days"
  } logged, mostly ${average.label.toLowerCase()}.`
}

/** Every other row is named, starting from whichever day the week starts on. */
function rowLabels(): string[] {
  const short = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
  const start = weekStartsOn()

  return short.map((_, row) => (row % 2 === 1 ? short[(row + start) % 7] : ""))
}
