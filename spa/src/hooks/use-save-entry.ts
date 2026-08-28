import { useMutation, useQueryClient } from "@tanstack/react-query"
import { ApiError, entries } from "@/api/client"
import { toCreateRequest } from "@/lib/entry-draft"
import type { EntryDraft } from "@/lib/entry-draft"
import { discardUploads, uploadDraftMedia } from "@/lib/media-upload"
import { enqueue } from "@/lib/store/outbox"
import { clearDraft } from "@/lib/store/draft-store"
import { toast } from "@/components/ui/toast"
import { invalidateJournal } from "./use-entries"
import { OUTBOX_KEY } from "./use-outbox"
import { useOnline } from "./use-online"

export interface SaveOutcome {
  /** True when the entry is waiting on the device rather than on the server. */
  queued: boolean
}

/**
 * A failure the server never saw. `handleError` only raises an ApiError when a
 * response came back, so anything else is the connection rather than a refusal
 * — and a refusal must not be retried behind the reader's back.
 */
function neverReachedServer(error: unknown): boolean {
  return !(error instanceof ApiError)
}

export function useSaveNewEntry() {
  const queryClient = useQueryClient()
  const online = useOnline()

  return useMutation({
    meta: { silent: true },
    mutationFn: async (draft: EntryDraft): Promise<SaveOutcome> => {
      if (!online) {
        await enqueue(draft)
        return { queued: true }
      }

      const receipt = await uploadDraftMedia(draft).catch(async (error) => {
        if (!neverReachedServer(error)) throw error
        return null
      })

      if (receipt === null) {
        await enqueue(draft)
        return { queued: true }
      }

      try {
        await entries.create(toCreateRequest(draft, receipt))
        return { queued: false }
      } catch (error) {
        await discardUploads(receipt)
        if (neverReachedServer(error)) {
          await enqueue(draft)
          return { queued: true }
        }
        throw error
      }
    },
    onSuccess: (outcome) => {
      clearDraft()
      queryClient.invalidateQueries({ queryKey: OUTBOX_KEY })

      if (outcome.queued) {
        toast.add({
          title: "Saved on this device",
          description: "It will send itself once you are back online.",
          type: "info",
        })
        return
      }

      invalidateJournal(queryClient)
      toast.add({ title: "Entry created", type: "success" })
    },
  })
}

export function useSaveEntryEdit(id: string) {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Entry updated" },
    mutationFn: async (draft: EntryDraft) => {
      const receipt = await uploadDraftMedia(draft)

      try {
        return await entries.update(id, toCreateRequest(draft, receipt))
      } catch (error) {
        await discardUploads(receipt)
        throw error
      }
    },
    onSuccess: () => invalidateJournal(queryClient),
  })
}
