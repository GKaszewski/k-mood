import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"

export function useCorrelation(activityId: string | null) {
  return useQuery({
    queryKey: ["correlation", activityId],
    queryFn: () => entries.correlation(activityId!),
    enabled: !!activityId,
  })
}

export function useAllCorrelations(activityIds: string[]) {
  return useQuery({
    queryKey: ["correlations", activityIds],
    queryFn: async () => {
      const results = await Promise.all(
        activityIds.map((id) => entries.correlation(id))
      )
      return results.filter((r) => r.correlation !== null)
    },
    enabled: activityIds.length > 0,
  })
}
