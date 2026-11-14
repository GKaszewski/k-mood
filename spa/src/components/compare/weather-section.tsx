import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ComparisonTable } from "@/components/compare/comparison-table"
import { capitalise, kindOfRow } from "@/lib/compare-format"
import type { WeatherComparisonRowResponse } from "@/api/schema"

function labelOf(row: WeatherComparisonRowResponse): string {
  if (row.condition === null) return "Average temperature"
  const condition = capitalise(row.condition)
  return row.key === "weatherShare"
    ? `${condition} days`
    : `Mood on ${row.condition} days`
}

export function WeatherSection({
  rows,
}: {
  rows: WeatherComparisonRowResponse[]
}) {
  const withData = rows.filter((row) => row.sampleA + row.sampleB > 0)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Weather</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        {withData.length ? (
          <>
            <p className="text-xs text-muted-foreground">
              Shares are of the days that have weather recorded, so a gap in
              observations does not read as a change in the weather.
            </p>
            <ComparisonTable
              subject="Weather"
              lines={withData.map((row) => ({
                id: `${row.key}:${row.condition ?? ""}`,
                label: labelOf(row),
                kind: kindOfRow(row.key),
                row,
                secondary: row.key === "moodInWeather",
              }))}
            />
          </>
        ) : (
          <p className="text-xs text-muted-foreground">
            No weather recorded in either period.
          </p>
        )}
      </CardContent>
    </Card>
  )
}
