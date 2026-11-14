import { useState } from "react"
import {
  Card,
  CardAction,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { ComparisonTable } from "@/components/compare/comparison-table"
import { SortToggle } from "@/components/compare/sort-toggle"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { groupRows, orderGroups } from "@/lib/compare-groups"
import type { GroupOrder } from "@/lib/compare-groups"
import { kindOfRow } from "@/lib/compare-format"
import type { MetricComparisonRowResponse } from "@/api/schema"

const LEAD_KEY = "metricMean"

const SECONDARY_LABELS: Record<string, string> = {
  metricDaysWithData: "days with data",
  metricTotal: "total",
}

function labelOf(row: MetricComparisonRowResponse): string {
  if (row.key !== LEAD_KEY) return SECONDARY_LABELS[row.key] ?? row.key
  const field = METRIC_FIELDS.find((candidate) => candidate.kind === row.kind)
  return `${field?.label ?? row.kind} per day`
}

export function MetricsSection({
  rows,
}: {
  rows: MetricComparisonRowResponse[]
}) {
  const [order, setOrder] = useState<GroupOrder>("server")
  const groups = orderGroups(
    groupRows(rows, (row) => row.kind, LEAD_KEY),
    order,
    "relative"
  ).filter((group) => group.rows.some((row) => row.sampleA + row.sampleB > 0))

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Health</CardTitle>
        {groups.length > 1 && (
          <CardAction>
            <SortToggle value={order} onChange={setOrder} />
          </CardAction>
        )}
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        {groups.length ? (
          <>
            <p className="text-xs text-muted-foreground">
              Averages over the days that have a reading. Totals depend on how
              many days had one, so they are shown for reference only.
            </p>
            <ComparisonTable
              subject="Metric"
              lines={groups.flatMap((group) =>
                group.rows.map((row) => ({
                  id: `${group.id}:${row.key}`,
                  label: labelOf(row),
                  kind: kindOfRow(row.key, row.unit),
                  row,
                  secondary: row !== group.lead,
                }))
              )}
            />
          </>
        ) : (
          <p className="text-xs text-muted-foreground">
            No health metrics recorded in either period.
          </p>
        )}
      </CardContent>
    </Card>
  )
}
