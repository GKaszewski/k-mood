import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { rangeOfYear } from "@/lib/instant-range"

export function useYearCalendar(year: number) {
  const { from, to } = rangeOfYear(year)

  return useQuery({
    queryKey: ["calendar", "year", year],
    queryFn: () => entries.calendar({ from, to }),
  })
}
