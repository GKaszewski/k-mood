import { useQuery, keepPreviousData } from "@tanstack/react-query"
import { subDays } from "date-fns"
import { entries } from "@/api/client"
import type { DaySpan } from "@/lib/compare-presets"
import { formatDate } from "@/lib/day"
import { PERIOD_DAYS } from "@/lib/period"
import type { Period } from "@/lib/period"
import { weekStartName } from "@/lib/week"
import { useMoodStats } from "./use-stats"

const TREND_FRESH_FOR = 60_000

function spanOfPeriod(period: Period): DaySpan | undefined {
  const days = PERIOD_DAYS[period]
  if (days === null) return undefined

  const today = new Date()
  return { from: formatDate(subDays(today, days)), to: formatDate(today) }
}

/**
 * The trend is bucketed by the server, which knows the calendar and the
 * reader's days; the page only says which span and which day a week starts on.
 */
export function useTrend(period: Period) {
  const { data: stats } = useMoodStats("all")
  const wholeHistory: DaySpan | undefined =
    stats?.firstLoggedOn && stats.lastLoggedOn
      ? { from: stats.firstLoggedOn, to: stats.lastLoggedOn }
      : undefined
  const span = period === "all" ? wholeHistory : spanOfPeriod(period)
  const weekStartsOn = weekStartName()

  return useQuery({
    queryKey: ["trend", period, span?.from, span?.to, weekStartsOn],
    enabled: span !== undefined,
    queryFn: () => entries.trend({ ...span!, weekStartsOn }),
    staleTime: TREND_FRESH_FOR,
    placeholderData: keepPreviousData,
  })
}
