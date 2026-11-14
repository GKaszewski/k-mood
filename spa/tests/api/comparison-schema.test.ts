import { describe, expect, test } from "vitest"
import { ComparisonResponse } from "@/api/schema"

/**
 * The wire shape of `GET /api/v1/comparisons`, written from the Rust response
 * structs in `crates/api-types/src/responses/comparison*.rs` (camelCase,
 * section rows flattened onto the base row, `Option` as `null`) and the
 * example in the Task 7 report — not from the zod schema it checks.
 */
function base(key: string, overrides: Record<string, unknown> = {}) {
  return {
    key,
    a: 0.25,
    b: 0.4,
    absoluteChange: 0.15,
    relativeChange: 0.6,
    sampleA: 20,
    sampleB: 24,
    significance: "noise",
    ...overrides,
  }
}

function wire(cycle: unknown) {
  return {
    a: {
      from: "2025-11-01",
      to: "2025-11-30",
      clipped: false,
      daysInSpan: 30,
      loggedDays: 20,
    },
    b: {
      from: "2026-11-01",
      to: "2026-11-14",
      clipped: true,
      daysInSpan: 14,
      loggedDays: 12,
    },
    spansOverlap: false,
    trend: {
      granularity: "day",
      a: [
        {
          offset: 0,
          start: "2025-11-01",
          daysCovered: 1,
          meanMood: 4.0,
          loggedDays: 1,
          entries: 2,
        },
        {
          offset: 1,
          start: "2025-11-02",
          daysCovered: 1,
          meanMood: null,
          loggedDays: 0,
          entries: 0,
        },
      ],
      b: [
        {
          offset: 0,
          start: "2026-11-01",
          daysCovered: 1,
          meanMood: 3.5,
          loggedDays: 1,
          entries: 1,
        },
      ],
    },
    overview: [
      base("daysInSpan", {
        a: 30.0,
        b: 14.0,
        absoluteChange: -16.0,
        relativeChange: -0.5333,
        significance: "notTested",
      }),
      base("coverage", {
        a: 0.667,
        b: 0.857,
        absoluteChange: 0.19,
        significance: "notTested",
      }),
    ],
    mood: {
      rows: [
        base("averageMood", {
          a: 3.4,
          b: 3.9,
          absoluteChange: 0.5,
          relativeChange: 0.147,
          significance: "likelyReal",
          mood: null,
          moodLabel: null,
        }),
        base("moodShare", { mood: 5, moodLabel: "Rad" }),
        base("moodShare", {
          a: 0.0,
          absoluteChange: 0.4,
          relativeChange: null,
          mood: 1,
          moodLabel: "Awful",
        }),
      ],
      distribution: {
        a: [{ mood: 1, moodLabel: "Awful", count: 0 }],
        b: [{ mood: 1, moodLabel: "Awful", count: 3 }],
      },
    },
    activities: [
      base("activityShare", {
        activityId: "6f1c2a7e-8f2b-4d0a-9c3e-1b2d3e4f5a6b",
        name: "Running",
        category: "Sport",
      }),
      base("moodWithActivity", {
        a: 4.1,
        b: null,
        absoluteChange: null,
        relativeChange: null,
        sampleB: 0,
        significance: "notEnoughData",
        activityId: "6f1c2a7e-8f2b-4d0a-9c3e-1b2d3e4f5a6b",
        name: "Running",
        category: null,
      }),
    ],
    metrics: [
      base("metricMean", {
        a: 422.0,
        b: 441.0,
        absoluteChange: 19.0,
        kind: "sleepMinutes",
        unit: "minutes",
      }),
      base("metricTotal", {
        significance: "notTested",
        kind: "steps",
        unit: "steps",
      }),
    ],
    weather: [
      base("weatherShare", { condition: "rain" }),
      base("meanTemperature", {
        a: 8.5,
        b: 6.0,
        absoluteChange: -2.5,
        condition: null,
      }),
    ],
    music: {
      a: {
        entriesWithSong: 4,
        artists: [{ name: "Low", count: 2, share: 0.5 }],
        songs: [{ artist: "Low", title: "Lullaby", count: 1, share: 0.25 }],
      },
      b: { entriesWithSong: 0, artists: [], songs: [] },
      onlyInA: ["Low"],
      onlyInB: [],
    },
    cycle,
  }
}

describe("comparison response schema", () => {
  test("parses flattened section rows and keeps their fields", () => {
    const parsed = ComparisonResponse.parse(
      wire([base("moodInCycleQuarter", { quarter: 3 })])
    )

    expect(parsed.b.clipped).toBe(true)
    expect(parsed.trend.a[1].meanMood).toBeNull()
    expect(parsed.mood.rows[1]).toMatchObject({ mood: 5, moodLabel: "Rad" })
    expect(parsed.mood.rows[0].mood).toBeNull()
    expect(parsed.activities[0]).toMatchObject({
      key: "activityShare",
      name: "Running",
      category: "Sport",
      significance: "noise",
    })
    expect(parsed.activities[1]).toMatchObject({
      b: null,
      absoluteChange: null,
      category: null,
      significance: "notEnoughData",
    })
    expect(parsed.metrics[0]).toMatchObject({
      kind: "sleepMinutes",
      unit: "minutes",
      a: 422,
    })
    expect(parsed.weather[1].condition).toBeNull()
    expect(parsed.music.onlyInA).toEqual(["Low"])
    expect(parsed.cycle?.[0].quarter).toBe(3)
  })

  test("accepts a user who does not track their cycle", () => {
    const parsed = ComparisonResponse.parse(wire(null))

    expect(parsed.cycle).toBeNull()
  })

  test("rejects a row whose significance the server never sends", () => {
    const broken = wire(null)
    broken.overview[0].significance = "maybe"

    expect(ComparisonResponse.safeParse(broken).success).toBe(false)
  })
})
