import { describe, expect, test } from "vitest"
import type { ComparedTrendPointResponse } from "@/api/schema"
import { barsOf } from "@/lib/trend-labels"

function point(start: string, meanMood: number | null, entries: number) {
  return {
    offset: 0,
    start,
    daysCovered: 1,
    meanMood,
    loggedDays: entries === 0 ? 0 : 1,
    entries,
  } satisfies ComparedTrendPointResponse
}

describe("barsOf", () => {
  test("a day is labelled by its date and its weekday in the tooltip", () => {
    const [bar] = barsOf("day", [point("2026-10-07", 4, 2)])

    expect(bar).toEqual({
      label: "Oct 7",
      tooltip: "Wed, Oct 7",
      average: 4,
      entryCount: 2,
    })
  })

  test("a week is labelled by the day it starts", () => {
    const [bar] = barsOf("week", [point("2026-10-04", 3.5, 5)])

    expect(bar.label).toBe("Oct 4")
    expect(bar.tooltip).toBe("Week of Oct 4, 2026")
  })

  test("a month is labelled by its name and year", () => {
    const [bar] = barsOf("month", [point("2026-10-01", 3, 1)])

    expect(bar.label).toBe("Oct 2026")
    expect(bar.tooltip).toBe("October 2026")
  })

  test("a bucket nothing was logged in is a gap", () => {
    const [bar] = barsOf("week", [point("2026-10-04", null, 0)])

    expect(bar.average).toBeNull()
    expect(bar.entryCount).toBe(0)
  })
})
