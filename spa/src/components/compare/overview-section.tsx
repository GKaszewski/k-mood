import { TriangleAlert } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ComparisonTable } from "@/components/compare/comparison-table"
import { kindOfRow } from "@/lib/compare-format"
import type { ComparisonRowResponse } from "@/api/schema"

/** Below this share of days logged, a side is too thin to say much about. */
const THIN_COVERAGE = 0.5
/** Coverage further apart than this makes the two sides hard to compare. */
const COVERAGE_GAP = 0.3

const LABELS: Record<string, string> = {
  daysInSpan: "Days",
  loggedDays: "Logged days",
  coverage: "Coverage",
  entries: "Entries",
  entriesPerLoggedDay: "Entries per logged day",
}

function coverageWarning(rows: ComparisonRowResponse[]): string | null {
  const coverage = rows.find((row) => row.key === "coverage")
  if (!coverage) return null

  const thin = [coverage.a, coverage.b].some(
    (side) => side !== null && side < THIN_COVERAGE
  )
  if (thin) {
    return "One period has fewer than half its days logged. Its figures rest on little, so read the differences with care."
  }

  const gap = coverage.absoluteChange
  if (gap !== null && Math.abs(gap) > COVERAGE_GAP) {
    return "The two periods were logged very differently. Figures are per logged day, but a sparsely logged period may only show the days that stood out."
  }

  return null
}

export function OverviewSection({ rows }: { rows: ComparisonRowResponse[] }) {
  const warning = coverageWarning(rows)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Overview</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {warning && (
          <Alert>
            <TriangleAlert aria-hidden="true" />
            <AlertTitle>Uneven logging</AlertTitle>
            <AlertDescription>{warning}</AlertDescription>
          </Alert>
        )}
        <ComparisonTable
          lines={rows.map((row) => ({
            id: row.key,
            label: LABELS[row.key] ?? row.key,
            kind: kindOfRow(row.key),
            row,
          }))}
        />
      </CardContent>
    </Card>
  )
}
