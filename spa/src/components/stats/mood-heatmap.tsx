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
import { getMood } from "@/lib/mood"
import type { CalendarDayResponse } from "@/api/schema"

interface MoodHeatmapProps {
  year: number
  days: CalendarDayResponse[]
}

const MOOD_BG: Record<number, string> = {
  1: "bg-red-500",
  2: "bg-orange-400",
  3: "bg-yellow-400",
  4: "bg-lime-400",
  5: "bg-green-500",
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
  const weekStart = startOfWeek(yearStart)
  const totalWeeks = differenceInWeeks(yearEnd, weekStart) + 1

  const grid: (Date | null)[][] = Array.from({ length: 7 }, () =>
    Array.from({ length: totalWeeks }, () => null)
  )

  for (const day of allDays) {
    const weekIndex = differenceInWeeks(day, weekStart)
    const dayIndex = getDay(day)
    grid[dayIndex][weekIndex] = day
  }

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood heatmap — {year}</CardTitle>
      </CardHeader>
      <CardContent className="overflow-x-auto">
        <div className="flex min-w-[640px] gap-0.5">
          <div className="flex flex-col gap-0.5 pr-1 text-[9px] text-muted-foreground">
            {["", "Mon", "", "Wed", "", "Fri", ""].map((label, i) => (
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

                  const dateStr = format(day, "yyyy-MM-dd")
                  const mood = moodMap.get(dateStr)

                  return (
                    <div
                      key={weekIdx}
                      className={cn(
                        "h-[11px] w-[11px] rounded-[2px]",
                        mood ? MOOD_BG[mood] : "bg-muted/30"
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
              className={cn("h-[11px] w-[11px] rounded-[2px]", MOOD_BG[m])}
              title={getMood(m).label}
            />
          ))}
          <span>Better</span>
        </div>
      </CardContent>
    </Card>
  )
}
