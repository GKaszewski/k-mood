import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { entries } from "@/api/client"
import type { CreateEntryRequest } from "@/api/schema"

export function useEntries(options?: { limit?: number; offset?: number }) {
  return useQuery({
    queryKey: ["entries", { limit: options?.limit, offset: options?.offset }],
    queryFn: () => entries.list(options),
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
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["entries"] })
      queryClient.invalidateQueries({ queryKey: ["stats"] })
    },
  })
}

export function useDeleteEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Entry deleted" },
    mutationFn: (id: string) => entries.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["entries"] })
      queryClient.invalidateQueries({ queryKey: ["stats"] })
    },
  })
}

export function useMoodStats(from?: string, to?: string) {
  return useQuery({
    queryKey: ["stats", { from, to }],
    queryFn: () => entries.stats(from && to ? { from, to } : undefined),
  })
}
