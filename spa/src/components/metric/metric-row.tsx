import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import type { DailyMetricResponse } from "@/api/schema"
import type { MetricField } from "./metric-fields"

type MetricRowProps = {
  field: MetricField
  stored: DailyMetricResponse | undefined
  draft: string
  invalid?: boolean
  onChange: (draft: string) => void
}

export function MetricRow({
  field,
  stored,
  draft,
  invalid = false,
  onChange,
}: MetricRowProps) {
  const Icon = field.icon
  // Ties the label and the note to the field. Without the first of these the
  // only name the input had was its placeholder, so every one of them
  // announced itself as "Not recorded".
  const inputId = `metric-${field.kind}`
  const noteId = `${inputId}-note`

  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={inputId} className="flex items-center gap-2 text-xs">
        <Icon
          className="h-3.5 w-3.5 text-muted-foreground"
          aria-hidden="true"
        />
        {field.label}
        <span className="text-muted-foreground">({field.unit})</span>
      </Label>
      <Input
        id={inputId}
        type="number"
        inputMode="numeric"
        min={field.min}
        max={field.max}
        placeholder="Not recorded"
        aria-invalid={invalid || undefined}
        aria-describedby={stored ? noteId : undefined}
        value={draft}
        onChange={(event) => onChange(event.target.value)}
      />
      {stored && (
        <p id={noteId} className="text-xs text-muted-foreground">
          {stored.provider
            ? `Reported by ${stored.provider}. Saving replaces it with your own; empty the field to clear it, and a later import may report it again.`
            : "Entered by you. Empty the field to clear it."}
        </p>
      )}
    </div>
  )
}
