export const PERIODS = ["week", "month", "year", "all"] as const

export type Period = (typeof PERIODS)[number]

export const PERIOD_LABELS: Record<Period, string> = {
  week: "Week",
  month: "Month",
  year: "Year",
  all: "All",
}

/** How many days back a period reaches, or null when it reaches all the way. */
export const PERIOD_DAYS: Record<Period, number | null> = {
  week: 7,
  month: 30,
  year: 365,
  all: null,
}
