import {
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query"
import type { QueryClient } from "@tanstack/react-query"
import { entries } from "@/api/client"
import type { CreateEntryRequest } from "@/api/schema"

export const ENTRY_PAGE_SIZE = 30

/** Everything a written entry can change. */
const JOURNAL_KEYS = [["entries"], ["stats"], ["trend"], ["calendar"]] as const

export function invalidateJournal(queryClient: QueryClient): void {
  for (const queryKey of JOURNAL_KEYS) {
    queryClient.invalidateQueries({ queryKey })
  }
}

/** A narrowing of the journal. Every field left out widens the selection. */
export interface EntryFilters {
  mood?: number
  activity?: string
  from?: string
  to?: string
}

export function useEntryFeed(filters: EntryFilters = {}) {
  return useInfiniteQuery({
    queryKey: ["entries", "feed", filters],
    initialPageParam: 0,
    queryFn: ({ pageParam }) =>
      entries.list({ ...filters, limit: ENTRY_PAGE_SIZE, offset: pageParam }),
    getNextPageParam: (page) =>
      page.hasMore ? page.offset + page.limit : undefined,
  })
}

export function useRecentEntries(limit: number) {
  return useQuery({
    queryKey: ["entries", "recent", limit],
    queryFn: () => entries.list({ limit }),
  })
}

export function useEntry(id: string) {
  return useQuery({
    queryKey: ["entries", id],
    queryFn: () => entries.get(id),
    enabled: !!id,
  })
}

export function useCreateEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Entry created" },
    mutationFn: (body: CreateEntryRequest) => entries.create(body),
    onSuccess: () => invalidateJournal(queryClient),
  })
}

export function useDeleteEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Entry deleted" },
    mutationFn: (id: string) => entries.delete(id),
    onSuccess: () => invalidateJournal(queryClient),
  })
}
