import { useState } from "react"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { MoodHeatmap } from "./mood-heatmap"
import { useYearCalendar } from "@/hooks/use-year-calendar"
import { useDateBounds } from "@/hooks/use-date-bounds"

export function HeatmapSection() {
  const thisYear = new Date().getFullYear()
  const [year, setYear] = useState(thisYear)
  const { data: days, isLoading } = useYearCalendar(year)
  const { data: bounds } = useDateBounds()

  const earliestYear = bounds?.oldest.getFullYear() ?? thisYear

  return (
    <section className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          size="icon"
          aria-label={`Show ${year - 1}`}
          onClick={() => setYear((current) => current - 1)}
          disabled={year <= earliestYear}
        >
          <ChevronLeft className="h-5 w-5" aria-hidden="true" />
        </Button>
        <span className="text-sm font-medium" aria-live="polite">
          {year}
        </span>
        <Button
          variant="ghost"
          size="icon"
          aria-label={`Show ${year + 1}`}
          onClick={() => setYear((current) => current + 1)}
          disabled={year >= thisYear}
        >
          <ChevronRight className="h-5 w-5" aria-hidden="true" />
        </Button>
      </div>

      {isLoading ? (
        <Skeleton className="h-32" />
      ) : days ? (
        <MoodHeatmap year={year} days={days} />
      ) : null}
    </section>
  )
}
