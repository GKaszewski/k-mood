import { useState } from "react"
import { createFileRoute } from "@tanstack/react-router"
import {
  format,
  addMonths,
  subMonths,
  isSameDay,
  isBefore,
  isAfter,
  startOfMonth,
} from "date-fns"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { cn } from "@/lib/utils"
import { useCalendar } from "@/hooks/use-calendar"
import { useDateBounds } from "@/hooks/use-date-bounds"
import { EntryCard } from "@/components/entry/entry-card"
import { MoodDot } from "@/components/mood/mood-dot"
import { Skeleton } from "@/components/ui/skeleton"
import type { CalendarDayResponse } from "@/api/schema"

export const Route = createFileRoute("/_auth/calendar")({
  component: CalendarPage,
})

function CalendarPage() {
  const [month, setMonth] = useState(new Date())
  const [selectedDate, setSelectedDate] = useState<Date | null>(null)
  const { data: days, isLoading } = useCalendar(month)
  const { data: bounds } = useDateBounds()

  const canGoBack = bounds?.oldest
    ? isAfter(startOfMonth(month), startOfMonth(bounds.oldest))
    : false
  const canGoForward = isBefore(startOfMonth(month), startOfMonth(new Date()))

  const selectedEntries = selectedDate
    ? (days?.find((d) => isSameDay(new Date(d.date), selectedDate))?.entries ??
      [])
    : []

  return (
    <div className="flex flex-col gap-4 py-6">
      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setMonth((m) => subMonths(m, 1))}
          disabled={!canGoBack}
        >
          <ChevronLeft className="h-5 w-5" />
        </Button>
        <h1 className="text-lg font-bold">{format(month, "MMMM yyyy")}</h1>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setMonth((m) => addMonths(m, 1))}
          disabled={!canGoForward}
        >
          <ChevronRight className="h-5 w-5" />
        </Button>
      </div>

      {isLoading ? (
        <Skeleton className="h-64" />
      ) : (
        <MoodCalendarGrid
          month={month}
          days={days ?? []}
          selectedDate={selectedDate}
          onSelectDate={setSelectedDate}
        />
      )}

      {selectedDate && (
        <section>
          <h2 className="mb-2 text-sm font-medium text-muted-foreground">
            {format(selectedDate, "EEEE, MMMM d")}
          </h2>
          {selectedEntries.length > 0 ? (
            <div className="flex flex-col gap-2">
              {selectedEntries.map((entry) => (
                <EntryCard key={entry.id} entry={entry} />
              ))}
            </div>
          ) : (
            <Card>
              <CardContent className="py-6 text-center text-sm text-muted-foreground">
                No entries for this day
              </CardContent>
            </Card>
          )}
        </section>
      )}
    </div>
  )
}

function MoodCalendarGrid({
  month,
  days,
  selectedDate,
  onSelectDate,
}: {
  month: Date
  days: CalendarDayResponse[]
  selectedDate: Date | null
  onSelectDate: (date: Date) => void
}) {
  const year = month.getFullYear()
  const monthIdx = month.getMonth()
  const firstDay = new Date(year, monthIdx, 1).getDay()
  const daysInMonth = new Date(year, monthIdx + 1, 0).getDate()

  const dayNames = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]

  const moodMap = new Map<string, number>()
  for (const day of days) {
    if (day.dominantMood) {
      moodMap.set(day.date, day.dominantMood)
    }
  }

  return (
    <Card>
      <CardContent className="p-3">
        <div className="grid grid-cols-7 gap-1 text-center">
          {dayNames.map((d) => (
            <div
              key={d}
              className="py-1 text-xs font-medium text-muted-foreground"
            >
              {d}
            </div>
          ))}
          {Array.from({ length: firstDay }).map((_, i) => (
            <div key={`empty-${i}`} />
          ))}
          {Array.from({ length: daysInMonth }).map((_, i) => {
            const dayNum = i + 1
            const date = new Date(year, monthIdx, dayNum)
            const dateStr = format(date, "yyyy-MM-dd")
            const mood = moodMap.get(dateStr)
            const isSelected = selectedDate && isSameDay(date, selectedDate)
            const isToday = isSameDay(date, new Date())

            return (
              <Button
                key={dayNum}
                variant="ghost"
                onClick={() => onSelectDate(date)}
                className={cn(
                  "flex h-auto flex-col items-center gap-0.5 rounded-lg px-0 py-1.5 text-sm",
                  isSelected &&
                    "bg-primary text-primary-foreground hover:bg-primary/90",
                  isToday && !isSelected && "bg-muted font-bold"
                )}
              >
                <span>{dayNum}</span>
                {mood ? <MoodDot value={mood} /> : <div className="h-2" />}
              </Button>
            )
          })}
        </div>
      </CardContent>
    </Card>
  )
}
