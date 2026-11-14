import {
  differenceInCalendarWeeks,
  eachDayOfInterval,
  getDay,
  startOfWeek,
} from "date-fns"
import { rowOfWeek, weekStartsOn } from "./week"

const DAYS_IN_WEEK = 7

/** Seven weekday rows by however many weeks the span touches; empty outside it. */
export interface HeatmapGrid {
  weeks: number
  grid: (Date | null)[][]
}

/**
 * Lays a span of days out as week columns, the way the year heatmap always has,
 * but for any span — Compare draws one per period, a month or five years long.
 */
export function heatmapGrid(start: Date, end: Date): HeatmapGrid {
  const firstWeek = startOfWeek(start, { weekStartsOn: weekStartsOn() })
  const weeks = weekOf(end, firstWeek) + 1

  const grid: (Date | null)[][] = Array.from({ length: DAYS_IN_WEEK }, () =>
    Array.from({ length: weeks }, () => null)
  )

  for (const day of eachDayOfInterval({ start, end })) {
    grid[rowOfWeek(getDay(day))][weekOf(day, firstWeek)] = day
  }

  return { weeks, grid }
}

function weekOf(day: Date, firstWeek: Date): number {
  return differenceInCalendarWeeks(day, firstWeek, {
    weekStartsOn: weekStartsOn(),
  })
}
