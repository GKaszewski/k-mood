import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { useActivities } from "@/hooks/use-activities"
import { useAllCorrelations } from "@/hooks/use-correlation"

export function ActivityCorrelations() {
  const { data: activities } = useActivities()
  const activeIds = (activities ?? [])
    .filter((a) => !a.archived)
    .map((a) => a.id)

  const { data: correlations, isLoading } = useAllCorrelations(activeIds)

  const activityMap = new Map(
    (activities ?? []).map((a) => [a.id, a.name])
  )

  const sorted = [...(correlations ?? [])].sort(
    (a, b) => Math.abs(b.correlation!) - Math.abs(a.correlation!)
  )

  if (isLoading) return <Skeleton className="h-32" />
  if (!sorted.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Activity impact on mood</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        {sorted.slice(0, 8).map((c) => (
          <CorrelationBar
            key={c.activityId}
            name={activityMap.get(c.activityId) ?? c.activityId}
            value={c.correlation!}
          />
        ))}
      </CardContent>
    </Card>
  )
}

function CorrelationBar({ name, value }: { name: string; value: number }) {
  const pct = Math.abs(value) * 100
  const positive = value > 0

  return (
    <div className="flex items-center gap-3">
      <span className="w-24 shrink-0 truncate text-xs">{name}</span>
      <div className="relative h-4 flex-1 rounded-full bg-muted/50">
        <div
          className={`absolute top-0 h-full rounded-full ${
            positive ? "left-1/2 bg-green-500/60" : "right-1/2 bg-red-500/60"
          }`}
          style={{ width: `${pct / 2}%` }}
        />
        <div className="absolute left-1/2 top-0 h-full w-px bg-muted-foreground/30" />
      </div>
      <span
        className={`w-12 shrink-0 text-right text-xs font-medium ${
          positive ? "text-green-400" : "text-red-400"
        }`}
      >
        {positive ? "+" : ""}
        {value.toFixed(2)}
      </span>
    </div>
  )
}
