import { OUTBOX, all, get, isAvailable, put, remove } from "./idb"
import type { EntryDraft } from "../entry-draft"

/** An entry written on this device that the server has not accepted yet. */
export interface QueuedEntry {
  id: string
  draft: EntryDraft
  queuedAt: string
  attempts: number
  lastError?: string
}

export function isSupported(): boolean {
  return isAvailable()
}

export async function enqueue(draft: EntryDraft): Promise<QueuedEntry> {
  const queued: QueuedEntry = {
    id: crypto.randomUUID(),
    draft,
    queuedAt: new Date().toISOString(),
    attempts: 0,
  }

  await put(OUTBOX, queued.id, queued)
  return queued
}

export async function queuedEntries(): Promise<QueuedEntry[]> {
  if (!isAvailable()) return []

  try {
    const entries = await all<QueuedEntry>(OUTBOX)
    return entries.sort((left, right) =>
      left.queuedAt.localeCompare(right.queuedAt)
    )
  } catch {
    return []
  }
}

export async function dequeue(id: string): Promise<void> {
  await remove(OUTBOX, id)
}

/**
 * Records what went wrong. `countsAgainstIt` is false when the request never
 * reached the server: a connection that is down says nothing about whether
 * this entry is acceptable, so it must not use up the entry's attempts.
 */
export async function noteAttempt(
  id: string,
  error: string,
  countsAgainstIt: boolean
): Promise<void> {
  const queued = await get<QueuedEntry>(OUTBOX, id)
  if (!queued) return

  await put(OUTBOX, id, {
    ...queued,
    attempts: countsAgainstIt ? queued.attempts + 1 : queued.attempts,
    lastError: error,
  })
}
