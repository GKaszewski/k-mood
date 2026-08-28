import {
  format,
  eachDayOfInterval,
  startOfYear,
  endOfYear,
  getDay,
  startOfWeek,
  differenceInWeeks,
} from "date-fns"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { cn } from "@/lib/utils"
import { getMood, nearestMood } from "@/lib/mood"
import { rowOfWeek, weekStartsOn } from "@/lib/week"
import { formatDate } from "@/lib/day"
import type { CalendarDayResponse } from "@/api/schema"

interface MoodHeatmapProps {
  year: number
  days: CalendarDayResponse[]
}

const MONTH_LABELS = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
]

export function MoodHeatmap({ year, days }: MoodHeatmapProps) {
  const yearStart = startOfYear(new Date(year, 0, 1))
  const yearEnd = endOfYear(new Date(year, 0, 1))

  const moodMap = new Map<string, number>()
  for (const day of days) {
    if (day.mood) {
      moodMap.set(day.date, day.mood)
    }
  }

  const allDays = eachDayOfInterval({ start: yearStart, end: yearEnd })
  const weekStart = startOfWeek(yearStart, { weekStartsOn: weekStartsOn() })
  const totalWeeks = differenceInWeeks(yearEnd, weekStart) + 1

  const grid: (Date | null)[][] = Array.from({ length: 7 }, () =>
    Array.from({ length: totalWeeks }, () => null)
  )

  for (const day of allDays) {
    const weekIndex = differenceInWeeks(day, weekStart)
    const dayIndex = rowOfWeek(getDay(day))
    grid[dayIndex][weekIndex] = day
  }

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood heatmap — {year}</CardTitle>
      </CardHeader>
      <CardContent className="overflow-x-auto">
        <p className="sr-only">{summarise(year, moodMap)}</p>
        <div aria-hidden="true" className="flex min-w-[640px] gap-0.5">
          <div className="flex flex-col gap-0.5 pr-1 text-[9px] text-muted-foreground">
            {rowLabels().map((label, i) => (
              <div key={i} className="h-[11px] leading-[11px]">
                {label}
              </div>
            ))}
          </div>

          <div className="flex flex-col gap-0.5">
            <div className="mb-0.5 flex gap-0.5">
              {Array.from({ length: totalWeeks }, (_, weekIdx) => {
                const firstDayInWeek =
                  grid[0][weekIdx] ?? grid[1][weekIdx] ?? grid[2][weekIdx]
                const showLabel =
                  firstDayInWeek && firstDayInWeek.getDate() <= 7
                return (
                  <div
                    key={weekIdx}
                    className="w-[11px] text-center text-[8px] leading-[11px] text-muted-foreground"
                  >
                    {showLabel ? MONTH_LABELS[firstDayInWeek.getMonth()] : ""}
                  </div>
                )
              })}
            </div>

            {[0, 1, 2, 3, 4, 5, 6].map((dayIdx) => (
              <div key={dayIdx} className="flex gap-0.5">
                {grid[dayIdx].map((day, weekIdx) => {
                  if (!day) {
                    return <div key={weekIdx} className="h-[11px] w-[11px]" />
                  }

                  const dateStr = formatDate(day)
                  const mood = moodMap.get(dateStr)

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
      </CardContent>
    </Card>
  )
}

/**
 * The grid is 365 coloured squares — as tab stops it would be unusable, and
 * read out one by one it would be meaningless. This says the same thing in a
 * sentence; the calendar is where a screen reader goes for a given day.
 */
function summarise(year: number, moods: Map<string, number>): string {
  const logged = moods.size

  if (logged === 0) return `Mood heatmap for ${year}. Nothing logged.`

  const total = Array.from(moods.values()).reduce((sum, mood) => sum + mood, 0)
  const average = nearestMood(total / logged)

  return `Mood heatmap for ${year}. ${logged} ${
    logged === 1 ? "day" : "days"
  } logged, mostly ${average.label.toLowerCase()}.`
}

/** Every other row is named, starting from whichever day the week starts on. */
function rowLabels(): string[] {
  const short = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
  const start = weekStartsOn()

  return short.map((_, row) => (row % 2 === 1 ? short[(row + start) % 7] : ""))
}
