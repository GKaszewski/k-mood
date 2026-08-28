import { useEffect, useRef, useState } from "react"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { flushOutbox, isStuck, retryQueued } from "@/lib/outbox-sync"
import { dequeue, enqueue, queuedEntries } from "@/lib/store/outbox"
import type { QueuedEntry } from "@/lib/store/outbox"
import type { EntryDraft } from "@/lib/entry-draft"
import { invalidateJournal } from "./use-entries"
import { useOnline } from "./use-online"

export const OUTBOX_KEY = ["outbox"] as const

export function useQueuedEntries() {
  return useQuery({
    queryKey: OUTBOX_KEY,
    queryFn: queuedEntries,
    staleTime: Infinity,
  })
}

export function useQueueEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Saved — it will send when you are back online" },
    mutationFn: (draft: EntryDraft) => enqueue(draft),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: OUTBOX_KEY }),
  })
}

export function useDiscardQueued() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Discarded" },
    mutationFn: (id: string) => dequeue(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: OUTBOX_KEY }),
  })
}

export function useRetryQueued() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (queued: QueuedEntry) => retryQueued(queued),
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: OUTBOX_KEY })
      invalidateJournal(queryClient)
    },
  })
}

/**
 * How long to wait after a failed pass before trying again. Without a pause
 * the failure invalidates the queue, the queue wakes this effect, and it flushes
 * straight into the same failure — a tight loop that burns an entry's attempts
 * in a second. Regaining a connection resets it to the front.
 */
const RETRY_AFTER_SECONDS = [5, 15, 60, 300]

function pauseAfter(failures: number): number {
  if (failures === 0) return 0

  const index = Math.min(failures, RETRY_AFTER_SECONDS.length) - 1
  return RETRY_AFTER_SECONDS[index] * 1000
}

/**
 * Drains the outbox whenever there is something worth sending and a connection
 * to send it over. Mounted once, in the authenticated layout.
 */
export function useOutboxSync() {
  const queryClient = useQueryClient()
  const online = useOnline()
  const { data: pending } = useQueuedEntries()
  const [failures, setFailures] = useState(0)
  const wasOnline = useRef(online)

  const sendable = pending?.filter((queued) => !isStuck(queued)).length ?? 0

  const flush = useMutation({
    meta: { silent: true },
    mutationFn: flushOutbox,
    onSuccess: (outcome) => {
      setFailures((count) => (outcome.failed > 0 ? count + 1 : 0))
      queryClient.invalidateQueries({ queryKey: OUTBOX_KEY })
      if (outcome.sent > 0) invalidateJournal(queryClient)
    },
    onError: () => setFailures((count) => count + 1),
  })

  const { mutate: startFlush, isPending } = flush

  // A connection that has just come back deserves an immediate try.
  useEffect(() => {
    if (online && !wasOnline.current) setFailures(0)
    wasOnline.current = online
  }, [online])

  useEffect(() => {
    if (!online || sendable === 0 || isPending) return

    const timer = setTimeout(() => startFlush(), pauseAfter(failures))
    return () => clearTimeout(timer)
  }, [online, sendable, failures, isPending, startFlush])
}
