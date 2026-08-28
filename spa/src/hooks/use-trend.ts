import { useQuery, keepPreviousData } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { rangeOfDays } from "@/lib/instant-range"
import type { Period } from "@/lib/period"
import { rangeOfPeriod, useMoodStats } from "./use-stats"

/** One day that was actually logged, and how much was logged that day. */
export interface TrendPoint {
  date: string
  mood: number
  entryCount: number
}

const TREND_FRESH_FOR = 60_000

/**
 * The trend reads the calendar rather than the entry list: the chart plots one
 * point per day whatever the period, and the calendar already answers in days,
 * so no period can outgrow a page of entries and be quietly truncated.
 */
export function useTrend(period: Period) {
  const { data: stats } = useMoodStats("all")
  const wholeHistory =
    stats?.firstLoggedOn && stats.lastLoggedOn
      ? rangeOfDays(new Date(stats.firstLoggedOn), new Date(stats.lastLoggedOn))
      : undefined
  const range = period === "all" ? wholeHistory : rangeOfPeriod(period)

  return useQuery({
    queryKey: ["trend", period, range?.from, range?.to],
    enabled: range !== undefined,
    queryFn: async (): Promise<TrendPoint[]> => {
      if (!range) return []

      const days = await entries.calendar(range)

      return days.flatMap((day) =>
        day.dayMood === null
          ? []
          : [
              {
                date: day.date,
                mood: day.dayMood,
                entryCount: day.entries.length,
              },
            ]
      )
    },
    staleTime: TREND_FRESH_FOR,
    placeholderData: keepPreviousData,
  })
}
