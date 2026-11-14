import { describe, expect, test } from "vitest"
import { groupRows, orderGroups } from "@/lib/compare-groups"
import type { ComparisonRowResponse } from "@/api/schema"

type Row = ComparisonRowResponse & { subject: string }

function row(
  subject: string,
  key: string,
  change: number | null,
  relative: number | null = null
): Row {
  return {
    subject,
    key,
    a: null,
    b: null,
    absoluteChange: change,
    relativeChange: relative,
    sampleA: 0,
    sampleB: 0,
    significance: "notTested",
  }
}

const rows = [
  row("walk", "moodWithActivity", 0.9),
  row("walk", "activityShare", 0.05),
  row("run", "activityShare", -0.2),
  row("run", "moodWithActivity", 0.1),
  row("read", "activityShare", null),
]

describe("grouping comparison rows", () => {
  test("keeps the server's subject order and leads with the headline row", () => {
    const groups = groupRows(rows, (r) => r.subject, "activityShare")

    expect(groups.map((g) => g.id)).toEqual(["walk", "run", "read"])
    expect(groups[0].rows.map((r) => r.key)).toEqual([
      "activityShare",
      "moodWithActivity",
    ])
  })

  test("sorts by the size of the lead row's change, empty changes last", () => {
    const groups = groupRows(rows, (r) => r.subject, "activityShare")

    expect(orderGroups(groups, "change").map((g) => g.id)).toEqual([
      "run",
      "walk",
      "read",
    ])
  })

  test("leaves the server order alone when asked to", () => {
    const groups = groupRows(rows, (r) => r.subject, "activityShare")

    expect(orderGroups(groups, "server")).toBe(groups)
  })

  test("ranks by relative change when units differ, empty changes last", () => {
    const metrics = [
      row("steps", "metricMean", 1500, 0.2),
      row("hrv", "metricMean", -12, -0.4),
      row("drinks", "metricMean", 0.5, null),
      row("sleep", "metricMean", 30, 0.1),
    ]
    const groups = groupRows(metrics, (r) => r.subject, "metricMean")

    expect(orderGroups(groups, "change", "relative").map((g) => g.id)).toEqual([
      "hrv",
      "steps",
      "sleep",
      "drinks",
    ])
    expect(orderGroups(groups, "change").map((g) => g.id)).toEqual([
      "steps",
      "sleep",
      "hrv",
      "drinks",
    ])
  })
})
