import { Link } from "@tanstack/react-router"
import { ChevronRight } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { formatMetric } from "@/components/metric/metric-value"
import type { MetricField } from "@/components/metric/metric-fields"
import { byKind, useDailyMetrics } from "@/hooks/use-metrics"
import { today } from "@/lib/day"
import type { DailyMetricResponse } from "@/api/schema"

const MOST_SHOWN = 3

/**
 * Today's readings, and the way in to record them. Always present: this is the
 * only route to the metrics screen, so hiding it when nothing is recorded
 * leaves an account that has never entered a reading unable to enter its
 * first one.
 */
export function MetricsCard() {
  const day = today()
  const { data } = useDailyMetrics(day, day)
  const recorded = byKind(data)

  const shown = METRIC_FIELDS.filter((field) => recorded.has(field.kind)).slice(
    0,
    MOST_SHOWN
  )

  return (
    <Link to="/metrics" aria-label="Daily metrics">
      <Card className="transition-colors hover:bg-muted/50">
        <CardContent className="flex items-center justify-between gap-3 py-4">
          {shown.length ? (
            <div className="flex flex-1 items-center justify-around">
              {shown.map((field) => (
                <Reading
                  key={field.kind}
                  field={field}
                  reading={recorded.get(field.kind)}
                />
              ))}
            </div>
          ) : (
            <Prompt />
          )}
          <ChevronRight
            className="h-4 w-4 shrink-0 text-muted-foreground"
            aria-hidden="true"
          />
        </CardContent>
      </Card>
    </Link>
  )
}

function Reading({
  field,
  reading,
}: {
  field: MetricField
  reading: DailyMetricResponse | undefined
}) {
  const Icon = field.icon

  return (
    <div className="flex flex-col items-center gap-1">
      <Icon className="h-4 w-4 text-muted-foreground" aria-hidden="true" />
      <span className="text-sm font-semibold tabular-nums">
        {reading ? formatMetric(field, reading.value) : "—"}
      </span>
      <span className="text-[10px] text-muted-foreground">{field.label}</span>
    </div>
  )
}

function Prompt() {
  const Icon = METRIC_FIELDS[0].icon

  return (
    <div className="flex items-center gap-2">
      <Icon className="h-4 w-4 text-muted-foreground" aria-hidden="true" />
      <span className="text-sm">Add today's metrics</span>
    </div>
  )
}
