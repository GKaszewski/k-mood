import type { MetricField } from "./metric-fields"

const MINUTES_PER_HOUR = 60

/**
 * A reading as it should be read back. "432" under a heading of "Sleep" says
 * very little; "7h 12m" says the thing itself.
 */
export function formatMetric(field: MetricField, value: number): string {
  if (field.unit !== "minutes") return value.toLocaleString()

  const hours = Math.floor(value / MINUTES_PER_HOUR)
  const minutes = value % MINUTES_PER_HOUR

  if (hours === 0) return `${minutes}m`
  if (minutes === 0) return `${hours}h`

  return `${hours}h ${minutes}m`
}
