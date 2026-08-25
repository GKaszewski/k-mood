import { useQuery } from "@tanstack/react-query"
import { entries } from "@/api/client"

export function useDateBounds() {
  return useQuery({
    queryKey: ["entries", "bounds"],
    queryFn: async () => {
      const all = await entries.list({ limit: 1, offset: 0 })
      if (!all.length) return null
      const newest = new Date(all[0].loggedAt)

      const stats = await entries.stats()
      const total = stats.totalEntries

      if (total <= 1) return { oldest: newest, newest }

      const last = await entries.list({ limit: 1, offset: total - 1 })
      const oldest = last.length ? new Date(last[0].loggedAt) : newest

      return { oldest, newest }
    },
    staleTime: 5 * 60_000,
  })
}
