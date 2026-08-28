import { useQuery, keepPreviousData } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { rangeOfLastDays } from "@/lib/instant-range"
import type { InstantRange } from "@/lib/instant-range"
import { PERIOD_DAYS } from "@/lib/period"
import type { Period } from "@/lib/period"

const STATS_FRESH_FOR = 60_000

export function rangeOfPeriod(period: Period): InstantRange | undefined {
  const days = PERIOD_DAYS[period]
  return days === null ? undefined : rangeOfLastDays(days)
}

export function useMoodStats(period: Period = "all") {
  return useQuery({
    queryKey: ["stats", period],
    queryFn: () => entries.stats(rangeOfPeriod(period)),
    staleTime: STATS_FRESH_FOR,
    placeholderData: keepPreviousData,
  })
}
