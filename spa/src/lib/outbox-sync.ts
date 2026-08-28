import { ApiError, entries } from "@/api/client"
import { toCreateRequest } from "./entry-draft"
import { discardUploads, uploadDraftMedia } from "./media-upload"
import { dequeue, noteAttempt, queuedEntries } from "./store/outbox"
import type { QueuedEntry } from "./store/outbox"

/**
 * How many times a *refused* entry is retried before it is left alone. Only
 * refusals count: the server has looked at the entry and said no, and asking
 * again will get the same answer. A failure that never reached the server is
 * not the entry's fault and is retried for as long as it takes.
 */
export const MAX_AUTOMATIC_ATTEMPTS = 5

export interface FlushOutcome {
  sent: number
  failed: number
}

export function isStuck(queued: QueuedEntry): boolean {
  return queued.attempts >= MAX_AUTOMATIC_ATTEMPTS
}

function describe(error: unknown): string {
  if (error instanceof ApiError) return error.message
  return "No connection to the server"
}

/** Whether the server saw the request and turned it down. */
function wasRefused(error: unknown): boolean {
  return error instanceof ApiError
}

async function send(queued: QueuedEntry): Promise<void> {
  const receipt = await uploadDraftMedia(queued.draft)

  try {
    await entries.create(toCreateRequest(queued.draft, receipt))
  } catch (error) {
    await discardUploads(receipt)
    throw error
  }

  await dequeue(queued.id)
}

/**
 * Sends what is waiting, oldest first, one at a time. Order is kept because a
 * journal read back out of order would be a different journal.
 */
export async function flushOutbox(): Promise<FlushOutcome> {
  const outcome: FlushOutcome = { sent: 0, failed: 0 }

  for (const queued of await queuedEntries()) {
    if (isStuck(queued)) continue

    try {
      await send(queued)
      outcome.sent += 1
    } catch (error) {
      await noteAttempt(queued.id, describe(error), wasRefused(error))
      outcome.failed += 1
    }
  }

  return outcome
}

/** Retries one entry the automatic pass has given up on. */
export async function retryQueued(queued: QueuedEntry): Promise<void> {
  try {
    await send(queued)
  } catch (error) {
    await noteAttempt(queued.id, describe(error), wasRefused(error))
    throw error
  }
}
