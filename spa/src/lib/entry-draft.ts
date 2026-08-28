import {
  activitiesOf,
  contentOf,
  locationOf,
  photosOf,
  songOf,
  voiceMemosOf,
} from "./dimensions"
import { existingItem, pendingFiles } from "./media"
import type { MediaItem } from "./media"
import { toLoggedAt } from "./logged-at"
import type { MoodValue } from "./mood"
import type {
  CreateEntryRequest,
  DimensionPayload,
  EntryResponse,
} from "@/api/schema"

export interface Coordinates {
  latitude: number
  longitude: number
}

export interface SongNote {
  title: string
  artist: string
}

/** Everything the entry form holds, before any of it reaches the server. */
export interface EntryDraft {
  mood: MoodValue | null
  loggedAt: string
  content: string
  activityIds: string[]
  photos: MediaItem[]
  voiceMemos: MediaItem[]
  song: SongNote
  location: Coordinates | null
}

const NO_SONG: SongNote = { title: "", artist: "" }

export function emptyDraft(): EntryDraft {
  return {
    mood: null,
    loggedAt: toLoggedAt(new Date()),
    content: "",
    activityIds: [],
    photos: [],
    voiceMemos: [],
    song: NO_SONG,
    location: null,
  }
}

export function draftOf(entry: EntryResponse): EntryDraft {
  return {
    mood: entry.mood as MoodValue,
    loggedAt: entry.loggedAt,
    content: contentOf(entry) ?? "",
    activityIds: activitiesOf(entry),
    photos: photosOf(entry).map(existingItem),
    voiceMemos: voiceMemosOf(entry).map(existingItem),
    song: songOf(entry) ?? NO_SONG,
    location: locationOf(entry),
  }
}

/**
 * Whether anything behind the "add more" disclosure has been filled in.
 * Activities are deliberately not counted: they sit in the always-visible part
 * of the form, and tagging one should not make the extras spring open.
 */
export function hasExtras(draft: EntryDraft): boolean {
  return (
    draft.content.trim() !== "" ||
    draft.photos.length > 0 ||
    draft.voiceMemos.length > 0 ||
    draft.location !== null ||
    draft.song.title.trim() !== "" ||
    draft.song.artist.trim() !== ""
  )
}

/** Whether losing this draft would lose anything the reader entered. */
export function isWorthKeeping(draft: EntryDraft): boolean {
  return draft.mood !== null || draft.activityIds.length > 0 || hasExtras(draft)
}

export function countsPendingUploads(draft: EntryDraft): number {
  return (
    pendingFiles(draft.photos).length + pendingFiles(draft.voiceMemos).length
  )
}

export interface UploadedMedia {
  photoIds: string[]
  voiceMemoIds: string[]
}

function dimensionsOf(
  draft: EntryDraft,
  uploaded: UploadedMedia
): DimensionPayload[] {
  const dimensions: DimensionPayload[] = []
  const text = draft.content.trim()

  if (text) dimensions.push({ kind: "content", text })

  if (draft.activityIds.length) {
    dimensions.push({ kind: "activities", ids: draft.activityIds })
  }

  if (uploaded.photoIds.length) {
    dimensions.push({ kind: "photos", ids: uploaded.photoIds })
  }

  if (uploaded.voiceMemoIds.length) {
    dimensions.push({ kind: "voiceMemos", ids: uploaded.voiceMemoIds })
  }

  if (draft.location) {
    dimensions.push({ kind: "location", ...draft.location })
  }

  const title = draft.song.title.trim()
  const artist = draft.song.artist.trim()
  if (title && artist) dimensions.push({ kind: "song", title, artist })

  return dimensions
}

export function toCreateRequest(
  draft: EntryDraft,
  uploaded: UploadedMedia
): CreateEntryRequest {
  if (draft.mood === null) {
    throw new Error("An entry cannot be written without a mood")
  }

  return {
    mood: draft.mood,
    loggedAt: draft.loggedAt,
    dimensions: dimensionsOf(draft, uploaded),
  }
}
