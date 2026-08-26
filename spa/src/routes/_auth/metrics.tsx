import { useEffect, useState } from "react"
import { createFileRoute, Link } from "@tanstack/react-router"
import { format } from "date-fns"
import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Skeleton } from "@/components/ui/skeleton"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { MetricRow } from "@/components/metric/metric-row"
import {
  byKind,
  useDailyMetrics,
  useSetDailyMetrics,
} from "@/hooks/use-metrics"
import type { MetricKind, MetricPayload } from "@/api/schema"

export const Route = createFileRoute("/_auth/metrics")({
  component: MetricsPage,
})

type Drafts = Record<string, string>

function storedDrafts(stored: Map<MetricKind, { value: number }>): Drafts {
  return Object.fromEntries(
    METRIC_FIELDS.map((field) => [
      field.kind,
      stored.has(field.kind) ? String(stored.get(field.kind)?.value) : "",
    ])
  )
}

function saveLabel(changes: number, cleared: number): string {
  if (changes === 0) return "Nothing to save"

  const plural = changes === 1 ? "" : "s"
  if (cleared === 0) return `Save ${changes} change${plural}`
  if (cleared === changes)
    return `Clear ${cleared} reading${cleared === 1 ? "" : "s"}`

  return `Save ${changes} change${plural}, ${cleared} cleared`
}

function MetricsPage() {
  const today = format(new Date(), "yyyy-MM-dd")
  const [date, setDate] = useState(today)
  const { data: stored, isLoading } = useDailyMetrics(date, date)
  const save = useSetDailyMetrics()

  const recorded = byKind(stored)
  const [drafts, setDrafts] = useState<Drafts>({})
  const [syncedFor, setSyncedFor] = useState<string | null>(null)

  useEffect(() => {
    if (!stored || syncedFor === date) return
    setDrafts(storedDrafts(byKind(stored)))
    setSyncedFor(date)
  }, [stored, date, syncedFor])

  const edited = METRIC_FIELDS.filter((field) => {
    const draft = drafts[field.kind] ?? ""
    const current = recorded.get(field.kind)
    return draft !== (current ? String(current.value) : "")
  })

  const invalid = edited.filter((field) => {
    const draft = drafts[field.kind] ?? ""
    if (draft === "") return false

    const count = Number(draft)
    return !Number.isInteger(count) || count < field.min || count > field.max
  })

  const changes: MetricPayload[] = edited
    .filter((field) => !invalid.includes(field))
    .map((field) => ({
      kind: field.kind,
      value: drafts[field.kind] === "" ? null : Number(drafts[field.kind]),
    }))

  const cleared = changes.filter((change) => change.value === null).length

  return (
    <div className="flex flex-col gap-6 py-6">
      <header className="flex items-center gap-2">
        <Link to="/">
          <Button variant="ghost" size="icon">
            <ArrowLeft className="h-5 w-5" />
          </Button>
        </Link>
        <h1 className="text-2xl font-bold">Daily metrics</h1>
      </header>

      <div className="flex flex-col gap-1.5">
        <Label className="text-xs">Day</Label>
        <Input
          type="date"
          value={date}
          max={today}
          onChange={(event) => setDate(event.target.value)}
        />
      </div>

      <Card>
        <CardContent className="flex flex-col gap-4 py-4">
          {isLoading ? (
            <>
              <Skeleton className="h-14" />
              <Skeleton className="h-14" />
              <Skeleton className="h-14" />
            </>
          ) : (
            METRIC_FIELDS.map((field) => (
              <MetricRow
                key={field.kind}
                field={field}
                stored={recorded.get(field.kind)}
                draft={drafts[field.kind] ?? ""}
                onChange={(draft) =>
                  setDrafts((current) => ({ ...current, [field.kind]: draft }))
                }
              />
            ))
          )}
        </CardContent>
      </Card>

      {invalid.length > 0 && (
        <p className="text-xs text-destructive">
          {invalid.map((field) => field.label).join(", ")}:{" "}
          {invalid.length === 1 ? "value is" : "values are"} outside the allowed
          range.
        </p>
      )}

      <Button
        disabled={changes.length === 0 || save.isPending}
        onClick={() =>
          save.mutate(
            { date, values: changes },
            { onSuccess: () => setSyncedFor(null) }
          )
        }
      >
        {save.isPending ? "Saving…" : saveLabel(changes.length, cleared)}
      </Button>

      {save.isError && (
        <p className="text-xs text-destructive">
          {save.error instanceof Error ? save.error.message : "Could not save"}
        </p>
      )}
    </div>
  )
}
