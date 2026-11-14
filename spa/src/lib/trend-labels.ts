import { format } from "date-fns"
import type { ComparedTrendPointResponse, TrendGranularity } from "@/api/schema"
import { parseDate } from "./day"

/** One plotted column: a bucket the server drew, with its labels. */
export interface TrendBar {
  label: string
  tooltip: string
  average: number | null
  entryCount: number
}

interface BucketFormats {
  label: string
  tooltip: (formattedStart: string) => string
  tooltipFormat: string
}

const FORMATS: Record<TrendGranularity, BucketFormats> = {
  month: {
    label: "MMM yyyy",
    tooltipFormat: "MMMM yyyy",
    tooltip: (start) => start,
  },
  week: {
    label: "MMM d",
    tooltipFormat: "MMM d, yyyy",
    tooltip: (start) => `Week of ${start}`,
  },
  day: {
    label: "MMM d",
    tooltipFormat: "EEE, MMM d",
    tooltip: (start) => start,
  },
}

export function barsOf(
  granularity: TrendGranularity,
  points: ComparedTrendPointResponse[]
): TrendBar[] {
  const formats = FORMATS[granularity]

  return points.map((point) => {
    const start = parseDate(point.start)

    return {
      label: format(start, formats.label),
      tooltip: formats.tooltip(format(start, formats.tooltipFormat)),
      average: point.meanMood,
      entryCount: point.entries,
    }
  })
}
