import { useMutation, useQueryClient } from "@tanstack/react-query"
import { entries } from "@/api/client"
import type { UpdateEntryRequest } from "@/api/schema"
import { invalidateJournal } from "./use-entries"

export function useUpdateEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Entry updated" },
    mutationFn: ({ id, body }: { id: string; body: UpdateEntryRequest }) =>
      entries.update(id, body),
    onSuccess: () => invalidateJournal(queryClient),
  })
}
