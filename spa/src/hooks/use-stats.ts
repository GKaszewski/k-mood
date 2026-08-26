import { useQuery, keepPreviousData } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { format, subDays } from "date-fns"

function dateRange(days: number) {
  const to = format(new Date(), "yyyy-MM-dd'T'23:59:59+00:00")
  const from = format(subDays(new Date(), days), "yyyy-MM-dd'T'00:00:00+00:00")
  return { from, to }
}

export function useMoodStats(
  period: "week" | "month" | "year" | "all" = "all"
) {
  const range =
    period === "week"
      ? dateRange(7)
      : period === "month"
        ? dateRange(30)
        : period === "year"
          ? dateRange(365)
          : undefined

  return useQuery({
    queryKey: ["stats", period],
    queryFn: () => entries.stats(range),
    staleTime: 60_000,
    placeholderData: keepPreviousData,
  })
}

export type TrendEntry = { loggedAt: string; mood: number }

export function usePeriodEntries(period: "week" | "month" | "year" | "all") {
  const range =
    period === "week"
      ? dateRange(7)
      : period === "month"
        ? dateRange(30)
        : period === "year"
          ? dateRange(365)
          : undefined

  const useCalendarEndpoint = period === "year" || period === "all"

  return useQuery<TrendEntry[]>({
    queryKey: ["entries", "period", period],
    queryFn: async () => {
      if (useCalendarEndpoint && range) {
        const days = await entries.calendar(range)
        return days.map((day) => ({
          loggedAt: day.date + "T12:00:00+00:00",
          mood: day.dayMood ?? 3,
        }))
      }

      const list = await entries.list(range)
      return list.map((e) => ({ loggedAt: e.loggedAt, mood: e.mood }))
    },
    staleTime: 60_000,
    placeholderData: keepPreviousData,
  })
}

export function useWeeklyMoods() {
  return usePeriodEntries("week")
}

export function useMonthlyMoods() {
  return usePeriodEntries("month")
}
