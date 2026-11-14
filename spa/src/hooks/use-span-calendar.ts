import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { rangeOfDays } from "@/lib/instant-range"
import { parseDate } from "@/lib/day"

/** The calendar days of any span, `from` and `to` as `yyyy-MM-dd`. */
export function useSpanCalendar(from: string, to: string) {
  return useQuery({
    queryKey: ["calendar", "span", from, to],
    queryFn: () =>
      entries.calendar(rangeOfDays(parseDate(from), parseDate(to))),
  })
}
