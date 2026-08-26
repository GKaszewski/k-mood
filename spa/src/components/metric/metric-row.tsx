import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import type { DailyMetricResponse } from "@/api/schema"
import type { MetricField } from "./metric-fields"

type MetricRowProps = {
  field: MetricField
  stored: DailyMetricResponse | undefined
  draft: string
  onChange: (draft: string) => void
}

export function MetricRow({ field, stored, draft, onChange }: MetricRowProps) {
  const Icon = field.icon

  return (
    <div className="flex flex-col gap-1.5">
      <Label className="flex items-center gap-2 text-xs">
        <Icon className="h-3.5 w-3.5 text-muted-foreground" />
        {field.label}
        <span className="text-muted-foreground">({field.unit})</span>
      </Label>
      <Input
        type="number"
        inputMode="numeric"
        min={field.min}
        max={field.max}
        placeholder="Not recorded"
        value={draft}
        onChange={(event) => onChange(event.target.value)}
      />
      {stored?.provider ? (
        <p className="text-xs text-muted-foreground">
          Reported by {stored.provider}. Saving replaces it with your own; empty
          the field to clear it, and a later import may report it again.
        </p>
      ) : (
        stored && (
          <p className="text-xs text-muted-foreground">
            Entered by you. Empty the field to clear it.
          </p>
        )
      )}
    </div>
  )
}
