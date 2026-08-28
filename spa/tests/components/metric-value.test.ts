import { describe, expect, it } from "vitest"
import { METRIC_FIELDS } from "@/components/metric/metric-fields"
import { formatMetric } from "@/components/metric/metric-value"

const field = (kind: string) =>
  METRIC_FIELDS.find((each) => each.kind === kind)!

describe("reading a metric back", () => {
  it("reads minutes as hours and minutes", () => {
    expect(formatMetric(field("sleepMinutes"), 432)).toBe("7h 12m")
  })

  it("drops the hours when there are none", () => {
    expect(formatMetric(field("exerciseMinutes"), 45)).toBe("45m")
  })

  it("drops the minutes when the hour is whole", () => {
    expect(formatMetric(field("sleepMinutes"), 480)).toBe("8h")
  })

  it("shows zero minutes as zero, not blank", () => {
    expect(formatMetric(field("awakeMinutes"), 0)).toBe("0m")
  })

  it("leaves counts alone but groups them", () => {
    expect(formatMetric(field("steps"), 8432)).toBe((8432).toLocaleString())
    expect(formatMetric(field("restingHeartRate"), 58)).toBe("58")
  })
})
