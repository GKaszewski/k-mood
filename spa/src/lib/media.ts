/**
 * A photo or voice memo attached to an entry: either one the server already
 * holds, or one picked on this device and not yet uploaded.
 */
export type MediaItem =
  | { kind: "existing"; id: string }
  | { kind: "pending"; localId: string; file: File }

export function newPendingItem(file: File): MediaItem {
  return { kind: "pending", localId: crypto.randomUUID(), file }
}

export function existingItem(id: string): MediaItem {
  return { kind: "existing", id }
}

export function existingIds(items: MediaItem[]): string[] {
  return items.flatMap((item) => (item.kind === "existing" ? [item.id] : []))
}

export function pendingFiles(items: MediaItem[]): File[] {
  return items.flatMap((item) => (item.kind === "pending" ? [item.file] : []))
}

export function keyOf(item: MediaItem): string {
  return item.kind === "existing" ? item.id : item.localId
}
