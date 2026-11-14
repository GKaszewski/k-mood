import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ComparisonTable } from "@/components/compare/comparison-table"
import { kindOfRow } from "@/lib/compare-format"
import type { CycleComparisonRowResponse } from "@/api/schema"

const ORDINALS = ["First", "Second", "Third", "Fourth"]

/** Present only when the User tracks their cycle; the server sends `null` otherwise. */
export function CycleSection({ rows }: { rows: CycleComparisonRowResponse[] }) {
  if (!rows.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Cycle</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        <p className="text-xs text-muted-foreground">
          Average mood in each quarter of the cycle.
        </p>
        <ComparisonTable
          subject="Quarter"
          lines={rows.map((row) => ({
            id: `${row.key}:${row.quarter}`,
            label: `${ORDINALS[row.quarter - 1] ?? row.quarter} quarter`,
            kind: kindOfRow(row.key),
            row,
          }))}
        />
      </CardContent>
    </Card>
  )
}
