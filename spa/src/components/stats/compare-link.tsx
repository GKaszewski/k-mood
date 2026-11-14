import { Link } from "@tanstack/react-router"
import { ChevronRight, GitCompareArrows } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"

/** The way in to comparing two periods, beside the way in to daily metrics. */
export function CompareLink() {
  return (
    <Link to="/compare">
      <Card className="transition-colors hover:bg-muted/50">
        <CardContent className="flex items-center justify-between gap-3 py-4">
          <div className="flex items-center gap-3">
            <GitCompareArrows
              className="h-4 w-4 text-muted-foreground"
              aria-hidden="true"
            />
            <div className="flex flex-col">
              <span className="text-sm">Compare periods</span>
              <span className="text-xs text-muted-foreground">
                This month against the same month last year, and more
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
