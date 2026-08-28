import { format, startOfMonth, startOfWeek } from "date-fns"
import type { TrendPoint } from "@/hooks/use-trend"
import type { Period } from "./period"
import { weekStartsOn } from "./week"

export type TrendBucket = "day" | "week" | "month"

/** One plotted column: the days it covers, averaged. */
export interface TrendBar {
  label: string
  tooltip: string
  average: number
  entryCount: number
}

const DAYS_BEFORE_MONTHLY_BUCKETS = 365

export function bucketFor(period: Period, dayCount: number): TrendBucket {
  if (period === "week" || period === "month") return "day"
  return dayCount > DAYS_BEFORE_MONTHLY_BUCKETS ? "month" : "week"
}

interface BucketLabels {
  key: string
  label: string
  tooltip: string
}

function labelsFor(day: Date, bucket: TrendBucket): BucketLabels {
  switch (bucket) {
    case "month":
      return {
        key: format(startOfMonth(day), "yyyy-MM"),
        label: format(day, "MMM yyyy"),
        tooltip: format(day, "MMMM yyyy"),
      }
    case "week": {
      const weekStart = startOfWeek(day, { weekStartsOn: weekStartsOn() })
      return {
        key: format(weekStart, "yyyy-MM-dd"),
        label: format(weekStart, "MMM d"),
        tooltip: `Week of ${format(weekStart, "MMM d, yyyy")}`,
      }
    }
    case "day":
      return {
        key: format(day, "yyyy-MM-dd"),
        label: format(day, "MMM d"),
        tooltip: format(day, "EEE, MMM d"),
      }
  }
}

interface Accumulator extends BucketLabels {
  moodTotal: number
  dayCount: number
  entryCount: number
}

export function aggregateTrend(
  points: TrendPoint[],
  bucket: TrendBucket
): TrendBar[] {
  const buckets = new Map<string, Accumulator>()

  for (const point of points) {
    const labels = labelsFor(new Date(point.date), bucket)
    const running = buckets.get(labels.key) ?? {
      ...labels,
      moodTotal: 0,
      dayCount: 0,
      entryCount: 0,
    }

    running.moodTotal += point.mood
    running.dayCount += 1
    running.entryCount += point.entryCount
    buckets.set(labels.key, running)
  }

  return Array.from(buckets.values())
    .sort((left, right) => left.key.localeCompare(right.key))
    .map(({ label, tooltip, moodTotal, dayCount, entryCount }) => ({
      label,
      tooltip,
      average: moodTotal / dayCount,
      entryCount,
    }))
}
