import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { rangeOfMonth } from "@/lib/instant-range"

export function useCalendar(month: Date) {
  const { from, to } = rangeOfMonth(month)

  return useQuery({
    queryKey: ["calendar", from, to],
    queryFn: () => entries.calendar({ from, to }),
  })
}
