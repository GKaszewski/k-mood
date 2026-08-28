import {
  endOfDay,
  endOfMonth,
  endOfYear,
  startOfDay,
  startOfMonth,
  startOfYear,
  subDays,
} from "date-fns"
import { toLoggedAt } from "./logged-at"

/**
 * A span of instants the API reads by. Day boundaries are the reader's own —
 * "this month" ends when the month ends where you are, not in UTC — so the
 * offset has to travel with the instant rather than being assumed.
 */
export interface InstantRange {
  from: string
  to: string
}

export function rangeOfDays(from: Date, to: Date): InstantRange {
  return { from: toLoggedAt(startOfDay(from)), to: toLoggedAt(endOfDay(to)) }
}

export function rangeOfMonth(month: Date): InstantRange {
  return rangeOfDays(startOfMonth(month), endOfMonth(month))
}

export function rangeOfYear(year: number): InstantRange {
  const anyDayThatYear = new Date(year, 0, 1)
  return rangeOfDays(startOfYear(anyDayThatYear), endOfYear(anyDayThatYear))
}

export function rangeOfLastDays(days: number): InstantRange {
  const today = new Date()
  return rangeOfDays(subDays(today, days), today)
}
