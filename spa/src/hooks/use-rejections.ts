import { useQuery } from "@tanstack/react-query"
import { metrics } from "@/api/client"

export function useRejections() {
  return useQuery({
    queryKey: ["metric-rejections"],
    queryFn: () => metrics.rejections(),
  })
}
