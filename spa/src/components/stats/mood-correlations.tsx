import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { useMoodCorrelations } from "@/hooks/use-mood-correlations"
import type { CorrelationRowResponse } from "@/api/schema"

const STRATEGY_LABELS: Record<string, string> = {
  pearson: "linear",
  spearman: "rank",
  kendall: "pairwise",
  meanDifference: "mean difference",
}

function labelOf(row: CorrelationRowResponse): string {
  const input = row.input

  switch (input.kind) {
    case "metric": {
      const field = METRIC_FIELDS.find((f) => f.kind === input.metric)
      return field ? field.label : input.metric
    }
    case "moonPhase":
      return "Moon phase"
    case "activity":
      return input.name
  }
}

function orderOf(row: CorrelationRowResponse): number {
  const input = row.input

  switch (input.kind) {
    case "metric":
      return METRIC_FIELDS.findIndex((f) => f.kind === input.metric)
    case "moonPhase":
      return METRIC_FIELDS.length
    case "activity":
      return METRIC_FIELDS.length + 1
  }
}

export function MoodCorrelations() {
  const { data: rows, isLoading } = useMoodCorrelations()

  if (isLoading) return <Skeleton className="h-40" />
  if (!rows) return null

  const withData = rows
    .filter((row) => row.sampleSize > 0)
    .sort((left, right) => orderOf(left) - orderOf(right))

  if (!withData.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">What moves with your mood</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <p className="text-xs text-muted-foreground">
          Over the last year. Several methods are run on each row and all of
          them are shown — how many point the same way matters more than any one
          number. Nothing here is ranked, and none of it is evidence on its own.
        </p>
        <p className="text-xs text-muted-foreground">
          A dot marks a reading that still stood once the number of things
          compared was accounted for. It is a second thing to look at, not a
          verdict.
        </p>

        {withData.map((row) => (
          <CorrelationRow key={rowKey(row)} row={row} />
        ))}
      </CardContent>
    </Card>
  )
}

function rowKey(row: CorrelationRowResponse): string {
  const input = row.input

  switch (input.kind) {
    case "metric":
      return `metric:${input.metric}`
    case "moonPhase":
      return "moonPhase"
    case "activity":
      return `activity:${input.activityId}`
  }
}

function CorrelationRow({ row }: { row: CorrelationRowResponse }) {
  const scored = row.scores.length > 0
  const isControl = row.input.kind === "moonPhase"

  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-baseline justify-between gap-3">
        <span className="truncate text-xs font-medium">{labelOf(row)}</span>
        <span className="shrink-0 text-xs text-muted-foreground">
          {agreementLine(row)}
        </span>
      </div>

      {scored ? (
        <div className="flex flex-wrap gap-x-3 gap-y-0.5">
          {row.scores.map((score) => (
            <span
              key={score.strategy}
              className="text-[10px] text-muted-foreground tabular-nums"
            >
              {STRATEGY_LABELS[score.strategy] ?? score.strategy}{" "}
              <span className="font-medium text-foreground">
                {score.coefficient.toFixed(2)}
              </span>
              {score.heldUp && (
                <span
                  className="ml-0.5 text-foreground"
                  title="still stood once the number of comparisons was accounted for"
                >
                  •
                </span>
              )}
            </span>
          ))}
        </div>
      ) : (
        <span className="text-[10px] text-muted-foreground">
          {row.sampleSize} days so far — not enough to say anything
        </span>
      )}

      {isControl && (
        <span className="text-[10px] text-muted-foreground">
          a control: the moon should show nothing, so a strong reading here
          means the rows above are noisier than they look
        </span>
      )}
    </div>
  )
}

function agreementLine(row: CorrelationRowResponse): string {
  if (row.agreement.applicable === 1) return "one measure only"
  if (row.scores.length === 0) return `${row.sampleSize} days`

  return `${row.agreement.agreeing} of ${row.agreement.applicable} agree`
}
