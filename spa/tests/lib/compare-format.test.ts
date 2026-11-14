import { describe, expect, test } from "vitest"
import {
  formatChange,
  formatRelative,
  formatSpan,
  formatValue,
  kindOfRow,
} from "@/lib/compare-format"

describe("comparison formatting", () => {
  test("mood averages get one decimal and the nearest mood's emoji", () => {
    expect(formatValue("mood", 3.94)).toBe("3.9 😊")
  })

  test("shares read as percentages and their changes as points", () => {
    expect(formatValue("share", 0.871)).toBe("87%")
    expect(formatChange("share", 0.07)).toBe("+7pp")
  })

  test("minutes read as hours and padded minutes", () => {
    expect(formatValue("minutes", 422)).toBe("7h 02m")
    expect(formatChange("minutes", -19)).toBe("−19m")
  })

  test("missing figures show a dash", () => {
    expect(formatValue("count", null)).toBe("—")
    expect(formatChange("mood", null)).toBe("—")
  })

  test("relative change is a signed whole percentage", () => {
    expect(formatRelative(0.123)).toBe("+12%")
    expect(formatRelative(-0.5)).toBe("−50%")
  })

  test("metric rows are written in their unit", () => {
    expect(kindOfRow("metricMean", "minutes")).toBe("minutes")
    expect(kindOfRow("metricMean", "steps")).toBe("number")
    expect(kindOfRow("activityShare")).toBe("share")
  })

  test("a clipped span says so", () => {
    expect(formatSpan("2026-01-01", "2026-11-14", true)).toBe(
      "1 Jan – 14 Nov 2026 (so far)"
    )
  })

  test("changes that round to zero carry no sign", () => {
    expect(formatChange("mood", 0.04)).toBe("0.0")
    expect(formatChange("share", -0.004)).toBe("0pp")
    expect(formatRelative(0.003)).toBe("0%")
    expect(formatChange("minutes", 0.2)).toBe("0m")
  })
})
