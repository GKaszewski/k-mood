import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"
import { format, startOfYear, endOfYear } from "date-fns"

export function useYearCalendar(year: number) {
  const from = format(
    startOfYear(new Date(year, 0, 1)),
    "yyyy-MM-dd'T'00:00:00+00:00"
  )
  const to = format(
    endOfYear(new Date(year, 0, 1)),
    "yyyy-MM-dd'T'23:59:59+00:00"
  )

  return useQuery({
    queryKey: ["calendar", "year", year],
    queryFn: () => entries.calendar({ from, to }),
  })
}
