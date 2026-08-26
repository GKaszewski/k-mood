import { format } from "date-fns"

const RFC3339_WITH_LOCAL_OFFSET = "yyyy-MM-dd'T'HH:mm:ssXXX"
const TIME_OF_DAY = "HH:mm"

export function toLoggedAt(moment: Date): string {
  return format(moment, RFC3339_WITH_LOCAL_OFFSET)
}

export function fromLoggedAt(value: string): Date {
  return new Date(value)
}

export function timeOfDayOf(moment: Date): string {
  return format(moment, TIME_OF_DAY)
}

export function withDayOf(moment: Date, day: Date): Date {
  const moved = new Date(moment)
  moved.setFullYear(day.getFullYear(), day.getMonth(), day.getDate())
  return moved
}

export function withTimeOfDay(moment: Date, timeOfDay: string): Date {
  const [hours, minutes] = timeOfDay.split(":").map(Number)
  if (!Number.isFinite(hours) || !Number.isFinite(minutes)) return moment

  const moved = new Date(moment)
  moved.setHours(hours, minutes, 0, 0)
  return moved
}
