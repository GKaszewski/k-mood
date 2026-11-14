import { format } from "date-fns"
import { nearestMood } from "@/lib/mood"
import { parseDate } from "@/lib/day"

/**
 * How a comparison figure reads. Every number arrives finished from the
 * server; this only decides how it is written down.
 */
export type ValueKind =
  "mood" | "share" | "count" | "decimal" | "minutes" | "temperature" | "number"

const PERCENT = 100
const MINUTES_PER_HOUR = 60
const LARGE_NUMBER = 1000
const MINUS = "−"
const EMPTY = "—"

const KIND_OF_KEY: Record<string, ValueKind> = {
  daysInSpan: "count",
  loggedDays: "count",
  coverage: "share",
  entries: "count",
  entriesPerLoggedDay: "decimal",
  averageMood: "mood",
  longestLoggingStreak: "count",
  longestGoodStreak: "count",
  moodShare: "share",
  activityShare: "share",
  moodWithActivity: "mood",
  metricDaysWithData: "count",
  weatherShare: "share",
  meanTemperature: "temperature",
  moodInWeather: "mood",
  moodInCycleQuarter: "mood",
}

/** Metric means and totals are written in their unit; minutes as `7h 02m`. */
export function kindOfRow(key: string, unit?: string): ValueKind {
  const known = KIND_OF_KEY[key] as ValueKind | undefined
  if (known) return known
  return unit === "minutes" ? "minutes" : "number"
}

const NONZERO_DIGIT = /[1-9]/

/** A sign only when the written figure is not zero: no `+0.0` or `−0pp`. */
function signed(value: number, written: string): string {
  if (!NONZERO_DIGIT.test(written)) return written
  return `${value > 0 ? "+" : value < 0 ? MINUS : ""}${written}`
}

function hoursAndMinutes(minutes: number): string {
  const whole = Math.round(minutes)
  const hours = Math.floor(whole / MINUTES_PER_HOUR)
  const rest = whole % MINUTES_PER_HOUR
  if (hours === 0) return `${rest}m`
  return `${hours}h ${rest.toString().padStart(2, "0")}m`
}

function plainNumber(value: number): string {
  return value.toLocaleString(undefined, {
    maximumFractionDigits: Math.abs(value) >= LARGE_NUMBER ? 0 : 1,
  })
}

function magnitude(kind: ValueKind, value: number): string {
  switch (kind) {
    case "mood":
    case "decimal":
      return value.toFixed(1)
    case "share":
      return Math.round(value * PERCENT).toString()
    case "count":
      return Math.round(value).toLocaleString()
    case "minutes":
      return hoursAndMinutes(value)
    case "temperature":
      return `${value.toFixed(1)}°C`
    case "number":
      return plainNumber(value)
  }
}

/** One side's figure: `3.9 😊`, `87%`, `7h 02m`. */
export function formatValue(kind: ValueKind, value: number | null): string {
  if (value === null) return EMPTY
  switch (kind) {
    case "mood":
      return `${magnitude(kind, value)} ${nearestMood(value).emoji}`
    case "share":
      return `${magnitude(kind, value)}%`
    default:
      return magnitude(kind, value)
  }
}

/** The change from A to B: `+0.5`, `+7pp`, `−19m`. */
export function formatChange(kind: ValueKind, change: number | null): string {
  if (change === null) return EMPTY
  const written = magnitude(kind, Math.abs(change))
  return `${signed(change, written)}${kind === "share" ? "pp" : ""}`
}

/** The change relative to A, as a secondary figure: `+12%`. */
export function formatRelative(change: number): string {
  return `${signed(change, Math.round(Math.abs(change) * PERCENT).toString())}%`
}

/** A fraction of something as a whole percentage: `31%`. */
export function formatShare(share: number): string {
  return `${Math.round(share * PERCENT)}%`
}

const SAME_YEAR_START = "d MMM"
const FULL_DAY = "d MMM yyyy"

/** `1 Nov – 30 Nov 2026`, with `(so far)` when the span was clipped to today. */
export function formatSpan(from: string, to: string, soFar = false): string {
  const start = parseDate(from)
  const end = parseDate(to)
  const startPattern =
    start.getFullYear() === end.getFullYear() ? SAME_YEAR_START : FULL_DAY
  const written = `${format(start, startPattern)} – ${format(end, FULL_DAY)}`
  return soFar ? `${written} (so far)` : written
}

/** `14 Nov 2026`. */
export function formatDay(day: string): string {
  return format(parseDate(day), FULL_DAY)
}

export function capitalise(word: string): string {
  return word.charAt(0).toUpperCase() + word.slice(1)
}
