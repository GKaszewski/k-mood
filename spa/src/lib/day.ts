import { format } from "date-fns"

const DAY = "yyyy-MM-dd"

/** A calendar day as the API keys day-scoped data by. */
export function formatDate(day: Date): string {
  return format(day, DAY)
}

export function today(): string {
  return formatDate(new Date())
}

export function parseDate(value: string): Date {
  const [year, month, day] = value.split("-").map(Number)
  return new Date(year, month - 1, day)
}
