import type { DimensionPayload, EntryResponse } from "@/api/schema"

function find<TKind extends DimensionPayload["kind"]>(
  entry: EntryResponse,
  kind: TKind
): Extract<DimensionPayload, { kind: TKind }> | undefined {
  return entry.dimensions.find((d) => d.kind === kind) as
    Extract<DimensionPayload, { kind: TKind }> | undefined
}

export function contentOf(entry: EntryResponse): string | null {
  return find(entry, "content")?.text ?? null
}

export function activitiesOf(entry: EntryResponse): string[] {
  return find(entry, "activities")?.ids ?? []
}

export function photosOf(entry: EntryResponse): string[] {
  return find(entry, "photos")?.ids ?? []
}

export function voiceMemosOf(entry: EntryResponse): string[] {
  return find(entry, "voiceMemos")?.ids ?? []
}

export function locationOf(entry: EntryResponse) {
  const location = find(entry, "location")
  return location
    ? { latitude: location.latitude, longitude: location.longitude }
    : null
}

export function songOf(entry: EntryResponse) {
  return find(entry, "song") ?? null
}

export function weatherOf(entry: EntryResponse) {
  return find(entry, "weather") ?? null
}

export function buildDimensions(input: {
  content: string
  activityIds: string[]
  photoIds: string[]
  voiceMemoIds: string[]
  location: { latitude: number; longitude: number } | null
  song: { title: string; artist: string } | null
}): DimensionPayload[] {
  const dimensions: DimensionPayload[] = []
  const text = input.content.trim()

  if (text) dimensions.push({ kind: "content", text })
  if (input.activityIds.length)
    dimensions.push({ kind: "activities", ids: input.activityIds })
  if (input.photoIds.length)
    dimensions.push({ kind: "photos", ids: input.photoIds })
  if (input.voiceMemoIds.length)
    dimensions.push({ kind: "voiceMemos", ids: input.voiceMemoIds })
  if (input.location)
    dimensions.push({
      kind: "location",
      latitude: input.location.latitude,
      longitude: input.location.longitude,
    })

  const title = input.song?.title.trim()
  const artist = input.song?.artist.trim()
  if (title && artist) dimensions.push({ kind: "song", title, artist })

  return dimensions
}
