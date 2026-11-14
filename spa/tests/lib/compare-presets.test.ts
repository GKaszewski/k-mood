import { describe, expect, test } from "vitest"
import {
  previousPeriod,
  sameMonthLastYear,
  thisSeasonVsLastSeason,
  thisYearVsLastYear,
} from "@/lib/compare-presets"

test("same month last year on 2026-11-14", () => {
  expect(sameMonthLastYear(new Date("2026-11-14"))).toEqual([
    { from: "2025-11-01", to: "2025-11-30" },
    { from: "2026-11-01", to: "2026-11-30" },
  ])
})

test("winter season spans the new year", () => {
  expect(thisSeasonVsLastSeason(new Date("2027-01-10"))[1]).toEqual({
    from: "2026-12-01",
    to: "2027-02-28",
  })
})

test("previous period has the same length and ends the day before", () => {
  expect(previousPeriod({ from: "2026-10-01", to: "2026-10-31" })).toEqual({
    from: "2026-08-31",
    to: "2026-09-30",
  })
})

describe("same month last year", () => {
  test("uses the whole month, including a leap February", () => {
    expect(sameMonthLastYear(new Date(2025, 1, 3))).toEqual([
      { from: "2024-02-01", to: "2024-02-29" },
      { from: "2025-02-01", to: "2025-02-28" },
    ])
  })
})

describe("this year vs last year", () => {
  test("compares whole calendar years, last year first", () => {
    expect(thisYearVsLastYear(new Date(2026, 10, 14))).toEqual([
      { from: "2025-01-01", to: "2025-12-31" },
      { from: "2026-01-01", to: "2026-12-31" },
    ])
  })
})

describe("this season vs last season", () => {
  test("compares a season with the same season a year earlier", () => {
    expect(thisSeasonVsLastSeason(new Date(2026, 6, 4))).toEqual([
      { from: "2025-06-01", to: "2025-08-31" },
      { from: "2026-06-01", to: "2026-08-31" },
    ])
  })

  test("a December day starts winter in its own year", () => {
    expect(thisSeasonVsLastSeason(new Date(2026, 11, 20))).toEqual([
      { from: "2025-12-01", to: "2026-02-28" },
      { from: "2026-12-01", to: "2027-02-28" },
    ])
  })

  test("winter's February follows the leap year", () => {
    expect(thisSeasonVsLastSeason(new Date(2024, 0, 15))[1]).toEqual({
      from: "2023-12-01",
      to: "2024-02-29",
    })
  })

  test("autumn is September to November", () => {
    expect(thisSeasonVsLastSeason(new Date(2026, 10, 14))[1]).toEqual({
      from: "2026-09-01",
      to: "2026-11-30",
    })
  })
})

describe("previous period", () => {
  test("a single day is preceded by the day before", () => {
    expect(previousPeriod({ from: "2026-03-01", to: "2026-03-01" })).toEqual({
      from: "2026-02-28",
      to: "2026-02-28",
    })
  })

  test("crosses a year boundary", () => {
    expect(previousPeriod({ from: "2026-01-01", to: "2026-01-07" })).toEqual({
      from: "2025-12-25",
      to: "2025-12-31",
    })
  })
})
