import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import {
  MoodHeatmapGrid,
  MoodHeatmapLegend,
} from "@/components/stats/mood-heatmap-grid"
import { useSpanCalendar } from "@/hooks/use-span-calendar"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import { formatSpan } from "@/lib/compare-format"
import { parseDate } from "@/lib/day"
import type { PeriodResponse } from "@/api/schema"

/**
 * Each period's days as a heatmap, A above B. Stacked rather than side by side
 * because a year is already as wide as the screen; both use the reader's week
 * start so the weekday rows line up.
 */
export function HeatmapSection({
  a,
  b,
}: {
  a: PeriodResponse
  b: PeriodResponse
}) {
  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood heatmap</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <Side side="A" color={PERIOD_A_COLOR} period={a} />
        <Side side="B" color={PERIOD_B_COLOR} period={b} />
        <MoodHeatmapLegend />
      </CardContent>
    </Card>
  )
}

function Side({
  side,
  color,
  period,
}: {
  side: "A" | "B"
  color: string
  period: PeriodResponse
}) {
  const { data: days, isLoading } = useSpanCalendar(period.from, period.to)

  return (
    <div className="flex flex-col gap-2">
      <h4 className="text-xs font-medium">
        <span aria-hidden="true" style={{ color }}>
          ●{" "}
        </span>
        {side}
        <span className="ml-1 font-normal text-muted-foreground">
          {formatSpan(period.from, period.to, period.clipped)}
        </span>
      </h4>
      {isLoading ? (
        <Skeleton className="h-24" />
      ) : days ? (
        <div className="overflow-x-auto pb-1">
          <MoodHeatmapGrid
            start={parseDate(period.from)}
            end={parseDate(period.to)}
            days={days}
            name={`period ${side}`}
          />
        </div>
      ) : null}
    </div>
  )
}
