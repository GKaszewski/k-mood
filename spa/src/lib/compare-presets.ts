import {
  addDays,
  addMonths,
  differenceInCalendarDays,
  endOfMonth,
  endOfYear,
  startOfMonth,
  startOfYear,
  subDays,
  subYears,
} from "date-fns"
import { formatDate, parseDate } from "@/lib/day"

/** Whole days, both ends inclusive, as `yyyy-MM-dd`. */
export type DaySpan = { from: string; to: string }

/** A comparison: A is the earlier/reference period, B the one compared with it. */
export type SpanPair = [DaySpan, DaySpan]

const MONTHS_PER_SEASON = 3
/** Meteorological seasons start in March, June, September and December. */
const SEASON_START_MONTHS = [2, 5, 8, 11] as const
const DECEMBER = 11

function spanOf(from: Date, to: Date): DaySpan {
  return { from: formatDate(from), to: formatDate(to) }
}

function monthOf(day: Date): DaySpan {
  return spanOf(startOfMonth(day), endOfMonth(day))
}

function yearOf(day: Date): DaySpan {
  return spanOf(startOfYear(day), endOfYear(day))
}

function seasonStart(day: Date): Date {
  const month = day.getMonth()
  const startMonth =
    [...SEASON_START_MONTHS].reverse().find((start) => start <= month) ??
    DECEMBER
  const year = startMonth > month ? day.getFullYear() - 1 : day.getFullYear()
  return new Date(year, startMonth, 1)
}

function seasonOf(day: Date): DaySpan {
  const start = seasonStart(day)
  const end = endOfMonth(addMonths(start, MONTHS_PER_SEASON - 1))
  return spanOf(start, end)
}

export function sameMonthLastYear(today: Date): SpanPair {
  return [monthOf(subYears(today, 1)), monthOf(today)]
}

/** Whole calendar years; the server clips this year's span to today. */
export function thisYearVsLastYear(today: Date): SpanPair {
  return [yearOf(subYears(today, 1)), yearOf(today)]
}

/** The current meteorological season against the same season a year earlier. */
export function thisSeasonVsLastSeason(today: Date): SpanPair {
  const current = seasonStart(today)
  return [seasonOf(subYears(current, 1)), seasonOf(current)]
}

/** The span of the same length that ends the day before `span` starts. */
export function previousPeriod(span: DaySpan): DaySpan {
  const from = parseDate(span.from)
  const length = differenceInCalendarDays(parseDate(span.to), from)
  const to = subDays(from, 1)
  return spanOf(addDays(to, -length), to)
}
