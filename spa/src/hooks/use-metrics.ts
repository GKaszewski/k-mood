import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { subDays } from "date-fns"
import { metrics } from "@/api/client"
import { formatDate } from "@/lib/day"
import type {
  DailyMetricResponse,
  MetricKind,
  MetricPayload,
} from "@/api/schema"

const METRICS = ["daily-metrics"]

export function useDailyMetrics(from: string, to: string) {
  return useQuery({
    queryKey: [...METRICS, from, to],
    queryFn: () => metrics.list({ from, to }),
  })
}

export function useSetDailyMetrics() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Metrics saved" },
    mutationFn: ({ date, values }: { date: string; values: MetricPayload[] }) =>
      metrics.set(date, values),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: METRICS }),
  })
}

export function byKind(
  stored: DailyMetricResponse[] | undefined
): Map<MetricKind, DailyMetricResponse> {
  return new Map(stored?.map((metric) => [metric.kind, metric]) ?? [])
}

const DAYS_THAT_MAKE_A_HABIT = 30

/**
 * Whether this account records daily metrics at all. Someone who has never
 * entered one should not be shown a card asking about their steps.
 */
export function useTracksMetrics(): boolean {
  const today = new Date()
  const { data } = useDailyMetrics(
    formatDate(subDays(today, DAYS_THAT_MAKE_A_HABIT)),
    formatDate(today)
  )

  return (data?.length ?? 0) > 0
}
