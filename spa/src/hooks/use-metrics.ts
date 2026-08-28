import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { metrics } from "@/api/client"
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
