import { useQuery } from "@tanstack/react-query"
import { format, subDays } from "date-fns"
import { correlations } from "@/api/client"

const A_YEAR = 365

export function useMoodCorrelations() {
  const to = format(new Date(), "yyyy-MM-dd")
  const from = format(subDays(new Date(), A_YEAR), "yyyy-MM-dd")

  return useQuery({
    queryKey: ["mood-correlations", from, to],
    queryFn: () => correlations.list({ from, to }),
  })
}
