import { describe, expect, it } from "vitest"
import { UNCATEGORISED, groupByCategory } from "@/lib/activity-groups"

const activity = (name: string, category: string | null) => ({ name, category })

describe("grouping activities", () => {
  it("sorts categories alphabetically", () => {
    const groups = groupByCategory([
      activity("run", "sport"),
      activity("read", "hobby"),
    ])

    expect(groups.map((group) => group.category)).toEqual(["hobby", "sport"])
  })

  it("puts activities with no category last", () => {
    const groups = groupByCategory([
      activity("nap", null),
      activity("run", "sport"),
      activity("read", "hobby"),
    ])

    expect(groups.map((group) => group.category)).toEqual([
      "hobby",
      "sport",
      UNCATEGORISED,
    ])
  })

  it("keeps activities within a category in the order given", () => {
    const groups = groupByCategory([
      activity("run", "sport"),
      activity("swim", "sport"),
    ])

    expect(groups[0].items.map((item) => item.name)).toEqual(["run", "swim"])
  })

  it("has no groups for no activities", () => {
    expect(groupByCategory([])).toEqual([])
  })
})
