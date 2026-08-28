import { media } from "@/api/client"
import { existingIds, pendingFiles } from "./media"
import type { EntryDraft, UploadedMedia } from "./entry-draft"

/** What an upload put on the server, and which of it this upload created. */
export interface UploadReceipt extends UploadedMedia {
  created: { photos: string[]; voiceMemos: string[] }
}

export const NOTHING_UPLOADED: UploadReceipt = {
  photoIds: [],
  voiceMemoIds: [],
  created: { photos: [], voiceMemos: [] },
}

/**
 * Media has to reach the server before the entry that references it, which
 * leaves a window where the upload has landed and the entry has not. Anything
 * this call created is handed back so the caller can take it away again if the
 * entry never arrives.
 */
export async function uploadDraftMedia(
  draft: EntryDraft
): Promise<UploadReceipt> {
  const photos: string[] = []
  const voiceMemos: string[] = []

  try {
    for (const file of pendingFiles(draft.photos)) {
      photos.push((await media.uploadPhoto(file)).id)
    }
    for (const file of pendingFiles(draft.voiceMemos)) {
      voiceMemos.push((await media.uploadVoiceMemo(file)).id)
    }
  } catch (error) {
    await discardUploads({ created: { photos, voiceMemos } })
    throw error
  }

  return {
    photoIds: [...existingIds(draft.photos), ...photos],
    voiceMemoIds: [...existingIds(draft.voiceMemos), ...voiceMemos],
    created: { photos, voiceMemos },
  }
}

export async function discardUploads(
  receipt: Pick<UploadReceipt, "created">
): Promise<void> {
  await Promise.allSettled([
    ...receipt.created.photos.map((id) => media.deletePhoto(id)),
    ...receipt.created.voiceMemos.map((id) => media.deleteVoiceMemo(id)),
  ])
}
