import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { format, startOfMonth, endOfMonth } from "date-fns"

export function useCalendar(month: Date) {
  const from = format(startOfMonth(month), "yyyy-MM-dd'T'00:00:00+00:00")
  const to = format(endOfMonth(month), "yyyy-MM-dd'T'23:59:59+00:00")

  return useQuery({
    queryKey: ["calendar", from, to],
    queryFn: () => entries.calendar({ from, to }),
  })
}
