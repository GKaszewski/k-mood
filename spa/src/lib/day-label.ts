import { format, isToday, isYesterday } from "date-fns"

const FULL_DAY = "EEEE, MMMM d"
const FULL_DAY_WITH_YEAR = "EEEE, MMMM d, yyyy"

/** A day named the way a person would name it. */
export function describeDay(day: Date): string {
  if (isToday(day)) return "Today"
  if (isYesterday(day)) return "Yesterday"

  const thisYear = day.getFullYear() === new Date().getFullYear()
  return format(day, thisYear ? FULL_DAY : FULL_DAY_WITH_YEAR)
}
