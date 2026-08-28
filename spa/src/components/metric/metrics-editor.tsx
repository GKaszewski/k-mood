import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { METRIC_FIELDS } from "./metric-fields"
import type { MetricField } from "./metric-fields"
import { MetricRow } from "./metric-row"
import { byKind, useSetDailyMetrics } from "@/hooks/use-metrics"
import { messageFor } from "@/lib/api-message"
import type { DailyMetricResponse, MetricPayload } from "@/api/schema"

type Drafts = Record<string, string>

function storedText(
  stored: Map<string, DailyMetricResponse>,
  field: MetricField
): string {
  const reading = stored.get(field.kind)
  return reading ? String(reading.value) : ""
}

function saveLabel(changes: number, cleared: number): string {
  if (changes === 0) return "Nothing to save"

  const plural = changes === 1 ? "" : "s"
  if (cleared === 0) return `Save ${changes} change${plural}`
  if (cleared === changes) {
    return `Clear ${cleared} reading${cleared === 1 ? "" : "s"}`
  }

  return `Save ${changes} change${plural}, ${cleared} cleared`
}

function isOutOfRange(field: MetricField, draft: string): boolean {
  if (draft === "") return false

  const count = Number(draft)
  return !Number.isInteger(count) || count < field.min || count > field.max
}

interface MetricsEditorProps {
  date: string
  stored: DailyMetricResponse[]
}

/**
 * One day's readings. Mounted fresh per day — the drafts are seeded from what
 * is stored, so there is no second copy of the day's state to keep in step.
 */
export function MetricsEditor({ date, stored }: MetricsEditorProps) {
  const recorded = byKind(stored)
  const [drafts, setDrafts] = useState<Drafts>(() =>
    Object.fromEntries(
      METRIC_FIELDS.map((field) => [field.kind, storedText(recorded, field)])
    )
  )
  const save = useSetDailyMetrics()

  const edited = METRIC_FIELDS.filter(
    (field) => (drafts[field.kind] ?? "") !== storedText(recorded, field)
  )
  const invalid = edited.filter((field) =>
    isOutOfRange(field, drafts[field.kind] ?? "")
  )

  const changes: MetricPayload[] = edited
    .filter((field) => !invalid.includes(field))
    .map((field) => ({
      kind: field.kind,
      value: drafts[field.kind] === "" ? null : Number(drafts[field.kind]),
    }))

  const cleared = changes.filter((change) => change.value === null).length

  return (
    <div className="flex flex-col gap-4">
      <Card>
        <CardContent className="flex flex-col gap-4 py-4">
          {METRIC_FIELDS.map((field) => (
            <MetricRow
              key={field.kind}
              field={field}
              stored={recorded.get(field.kind)}
              draft={drafts[field.kind] ?? ""}
              invalid={invalid.includes(field)}
              onChange={(draft) =>
                setDrafts((current) => ({ ...current, [field.kind]: draft }))
              }
            />
          ))}
        </CardContent>
      </Card>

      {invalid.length > 0 && (
        <p role="alert" className="text-xs text-destructive">
          {invalid.map((field) => field.label).join(", ")}:{" "}
          {invalid.length === 1 ? "value is" : "values are"} outside the allowed
          range.
        </p>
      )}

      <Button
        disabled={changes.length === 0 || save.isPending}
        onClick={() => save.mutate({ date, values: changes })}
      >
        {save.isPending ? "Saving…" : saveLabel(changes.length, cleared)}
      </Button>

      {save.isError && (
        <p role="alert" className="text-xs text-destructive">
          {messageFor(save.error)}
        </p>
      )}
    </div>
  )
}
