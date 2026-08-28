import { Link } from "@tanstack/react-router"
import { ChevronRight, LineChart } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"

/**
 * The second way in to daily metrics. They belong with the statistics — and a
 * single entry point on the dashboard once meant that an account with nothing
 * recorded could not reach the screen at all.
 */
export function MetricsLink() {
  return (
    <Link to="/metrics">
      <Card className="transition-colors hover:bg-muted/50">
        <CardContent className="flex items-center justify-between gap-3 py-4">
          <div className="flex items-center gap-3">
            <LineChart
              className="h-4 w-4 text-muted-foreground"
              aria-hidden="true"
            />
            <div className="flex flex-col">
              <span className="text-sm">Daily metrics</span>
              <span className="text-xs text-muted-foreground">
                Steps, sleep, heart rate and the rest
              </span>
            </div>
          </div>
          <ChevronRight
            className="h-4 w-4 shrink-0 text-muted-foreground"
            aria-hidden="true"
          />
        </CardContent>
      </Card>
    </Link>
  )
}
