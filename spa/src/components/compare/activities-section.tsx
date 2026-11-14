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
import { groupRows, orderGroups } from "@/lib/compare-groups"
import type { GroupOrder } from "@/lib/compare-groups"
import { kindOfRow } from "@/lib/compare-format"
import type { ActivityComparisonRowResponse } from "@/api/schema"

const LEAD_KEY = "activityShare"

function labelOf(row: ActivityComparisonRowResponse): string {
  return row.key === LEAD_KEY ? row.name : "mood on those days"
}

export function ActivitiesSection({
  rows,
}: {
  rows: ActivityComparisonRowResponse[]
}) {
  const [order, setOrder] = useState<GroupOrder>("server")
  const groups = orderGroups(
    groupRows(rows, (row) => row.activityId, LEAD_KEY),
    order
  )

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Activities</CardTitle>
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
              Share of logged days with each activity, and the average mood on
              those days.
            </p>
            <ComparisonTable
              subject="Activity"
              lines={groups.flatMap((group) =>
                group.rows.map((row) => ({
                  id: `${group.id}:${row.key}`,
                  label: labelOf(row),
                  kind: kindOfRow(row.key),
                  row,
                  secondary: row !== group.lead,
                }))
              )}
            />
          </>
        ) : (
          <p className="text-xs text-muted-foreground">
            No activities logged in either period.
          </p>
        )}
      </CardContent>
    </Card>
  )
}
