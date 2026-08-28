import { Link } from "@tanstack/react-router"
import { ChevronRight } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { byKind, useDailyMetrics, useTracksMetrics } from "@/hooks/use-metrics"
import { today } from "@/lib/day"

const MOST_SHOWN = 3

/** Today's readings, for accounts that record them. */
export function MetricsCard() {
  const day = today()
  const tracks = useTracksMetrics()
  const { data } = useDailyMetrics(day, day)
  const recorded = byKind(data)

  if (!tracks) return null

  const shown = METRIC_FIELDS.filter((field) => recorded.has(field.kind)).slice(
    0,
    MOST_SHOWN
  )

  return (
    <Link to="/metrics" aria-label="Daily metrics">
      <Card>
        <CardContent className="flex items-center justify-between gap-3 py-4">
          {shown.length ? (
            <div className="flex flex-1 items-center justify-around">
              {shown.map((field) => {
                const Icon = field.icon
                return (
                  <div
                    key={field.kind}
                    className="flex flex-col items-center gap-1"
                  >
                    <Icon
                      className="h-4 w-4 text-muted-foreground"
                      aria-hidden="true"
                    />
                    <span className="text-sm font-semibold tabular-nums">
                      {recorded.get(field.kind)?.value.toLocaleString()}
                    </span>
                    <span className="text-[10px] text-muted-foreground">
                      {field.label}
                    </span>
                  </div>
                )
              })}
            </div>
          ) : (
            <span className="text-sm text-muted-foreground">
              No readings today
            </span>
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
