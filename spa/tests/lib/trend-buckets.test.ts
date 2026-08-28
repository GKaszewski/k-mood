import { describe, expect, it } from "vitest"
import { aggregateTrend, bucketFor } from "@/lib/trend-buckets"
import type { TrendPoint } from "@/hooks/use-trend"

function day(date: string, mood: number, entryCount = 1): TrendPoint {
  return { date, mood, entryCount }
}

describe("trend buckets", () => {
  it("plots a day at a time over a week or a month", () => {
    expect(bucketFor("week", 7)).toBe("day")
    expect(bucketFor("month", 30)).toBe("day")
  })

  it("widens to weeks for a year and months beyond one", () => {
    expect(bucketFor("year", 300)).toBe("week")
    expect(bucketFor("all", 900)).toBe("month")
  })

  it("averages the days inside a bucket", () => {
    const bars = aggregateTrend(
      [day("2026-03-01", 2), day("2026-03-15", 4)],
      "month"
    )

    expect(bars).toHaveLength(1)
    expect(bars[0].average).toBe(3)
  })

  it("sums the entries behind a bucket, not the days", () => {
    const bars = aggregateTrend(
      [day("2026-03-01", 2, 3), day("2026-03-15", 4, 2)],
      "month"
    )

    expect(bars[0].entryCount).toBe(5)
  })

  it("orders buckets oldest first", () => {
    const bars = aggregateTrend(
      [day("2026-05-02", 3), day("2026-01-02", 5), day("2026-03-02", 1)],
      "month"
    )

    expect(bars.map((bar) => bar.average)).toEqual([5, 1, 3])
  })

  it("has nothing to plot for no days", () => {
    expect(aggregateTrend([], "day")).toEqual([])
  })
})
