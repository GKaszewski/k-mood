import { describe, expect, it } from "vitest"
import { heatmapGrid } from "@/lib/heatmap-grid"
import { formatDate } from "@/lib/day"
import { rowOfWeek } from "@/lib/week"

function placed(grid: (Date | null)[][]): Date[] {
  return grid.flat().filter((day): day is Date => day !== null)
}

describe("heatmap grid", () => {
  it("places every day of the span exactly once", () => {
    const { grid } = heatmapGrid(new Date(2026, 0, 1), new Date(2026, 11, 31))

    expect(placed(grid)).toHaveLength(365)
  })

  it("has seven rows of equal length", () => {
    const { grid, weeks } = heatmapGrid(
      new Date(2026, 9, 1),
      new Date(2026, 9, 31)
    )

    expect(grid).toHaveLength(7)
    for (const row of grid) expect(row).toHaveLength(weeks)
  })

  it("starts a span on its own weekday row, not the top", () => {
    const start = new Date(2026, 9, 7)
    const { grid } = heatmapGrid(start, new Date(2026, 9, 20))

    expect(formatDate(grid[rowOfWeek(start.getDay())][0]!)).toBe("2026-10-07")
  })

  it("leaves days outside the span empty", () => {
    const { grid } = heatmapGrid(new Date(2026, 9, 7), new Date(2026, 9, 20))
    const days = placed(grid).map(formatDate).sort()

    expect(days[0]).toBe("2026-10-07")
    expect(days).toHaveLength(14)
    expect(days).not.toContain("2026-10-06")
    expect(days).not.toContain("2026-10-21")
  })

  it("covers a single day with a single week", () => {
    const day = new Date(2026, 9, 7)
    const { grid, weeks } = heatmapGrid(day, day)

    expect(weeks).toBe(1)
    expect(placed(grid)).toHaveLength(1)
  })

  it("orders weeks left to right", () => {
    const { grid } = heatmapGrid(new Date(2026, 0, 1), new Date(2026, 1, 28))
    const firstRow = grid[0].filter((day): day is Date => day !== null)

    for (let i = 1; i < firstRow.length; i++) {
      expect(firstRow[i].getTime()).toBeGreaterThan(firstRow[i - 1].getTime())
    }
  })
})
