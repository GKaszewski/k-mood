import { describe, expect, it } from "vitest"
import { rangeOfDays, rangeOfMonth, rangeOfYear } from "@/lib/instant-range"

const OFFSET = /[+-]\d{2}:\d{2}$/

describe("instant ranges", () => {
  it("carries the reader's own offset rather than assuming UTC", () => {
    const { from, to } = rangeOfMonth(new Date(2026, 7, 15))

    expect(from).toMatch(OFFSET)
    expect(to).toMatch(OFFSET)
  })

  it("starts a month at local midnight on the first", () => {
    const { from } = rangeOfMonth(new Date(2026, 7, 15))

    expect(from.startsWith("2026-08-01T00:00:00")).toBe(true)
  })

  it("ends a month on the last day, not the first of the next", () => {
    const { to } = rangeOfMonth(new Date(2026, 7, 15))

    expect(to.startsWith("2026-08-31T23:59:59")).toBe(true)
  })

  it("covers a whole year", () => {
    const { from, to } = rangeOfYear(2025)

    expect(from.startsWith("2025-01-01T00:00:00")).toBe(true)
    expect(to.startsWith("2025-12-31T23:59:59")).toBe(true)
  })

  it("widens a span to whole days at both ends", () => {
    const { from, to } = rangeOfDays(
      new Date(2026, 0, 5, 13, 30),
      new Date(2026, 0, 7, 9, 15)
    )

    expect(from.startsWith("2026-01-05T00:00:00")).toBe(true)
    expect(to.startsWith("2026-01-07T23:59:59")).toBe(true)
  })
})
