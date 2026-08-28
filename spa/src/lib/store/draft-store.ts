import { DRAFTS, get, isAvailable, put, remove } from "./idb"
import type { EntryDraft } from "../entry-draft"

/**
 * Only the unsent new entry is kept. An edit in progress belongs to the entry
 * it is editing, and resurrecting one days later would be a surprise.
 */
const NEW_ENTRY = "new-entry"

export async function saveDraft(draft: EntryDraft): Promise<void> {
  if (!isAvailable()) return

  try {
    await put(DRAFTS, NEW_ENTRY, draft)
  } catch {
    // A draft that cannot be stored is not worth failing the form over.
  }
}

export async function loadDraft(): Promise<EntryDraft | undefined> {
  if (!isAvailable()) return undefined

  try {
    return await get<EntryDraft>(DRAFTS, NEW_ENTRY)
  } catch {
    return undefined
  }
}

export async function clearDraft(): Promise<void> {
  if (!isAvailable()) return

  try {
    await remove(DRAFTS, NEW_ENTRY)
  } catch {
    // Nothing to clear.
  }
}
