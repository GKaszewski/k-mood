import { describe, expect, it } from "vitest"
import { rowOfWeek, weekStartsOn, weekdayInitials } from "@/lib/week"

describe("week start", () => {
  it("names seven weekdays beginning with the first day", () => {
    const initials = weekdayInitials()

    expect(initials).toHaveLength(7)
    expect(new Set(initials).size).toBe(7)
  })

  it("puts the week's first day in the first row", () => {
    expect(rowOfWeek(weekStartsOn())).toBe(0)
  })

  it("maps every weekday to a distinct row", () => {
    const rows = [0, 1, 2, 3, 4, 5, 6].map(rowOfWeek)

    expect(new Set(rows)).toEqual(new Set([0, 1, 2, 3, 4, 5, 6]))
  })
})
