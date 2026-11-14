import { startOfYear, endOfYear } from "date-fns"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { MoodHeatmapGrid, MoodHeatmapLegend } from "./mood-heatmap-grid"
import type { CalendarDayResponse } from "@/api/schema"

interface MoodHeatmapProps {
  year: number
  days: CalendarDayResponse[]
}

export function MoodHeatmap({ year, days }: MoodHeatmapProps) {
  const anyDayThatYear = new Date(year, 0, 1)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood heatmap — {year}</CardTitle>
      </CardHeader>
      <CardContent className="overflow-x-auto">
        <div className="min-w-[640px]">
          <MoodHeatmapGrid
            start={startOfYear(anyDayThatYear)}
            end={endOfYear(anyDayThatYear)}
            days={days}
            name={String(year)}
          />
        </div>
        <MoodHeatmapLegend />
      </CardContent>
    </Card>
  )
}
