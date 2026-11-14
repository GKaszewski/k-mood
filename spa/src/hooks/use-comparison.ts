import { keepPreviousData, useQuery } from "@tanstack/react-query"
import { comparisons } from "@/api/client"
import type { DaySpan } from "@/lib/compare-presets"

const COMPARISON_FRESH_FOR = 60_000

/**
 * The whole comparison package for two spans. The previous answer stays on
 * screen while a new pair loads, so switching presets does not flash skeletons.
 */
export function useComparison(a: DaySpan, b: DaySpan) {
  return useQuery({
    queryKey: ["comparison", a, b],
    queryFn: () =>
      comparisons.compare({
        aFrom: a.from,
        aTo: a.to,
        bFrom: b.from,
        bTo: b.to,
      }),
    staleTime: COMPARISON_FRESH_FOR,
    placeholderData: keepPreviousData,
  })
}
