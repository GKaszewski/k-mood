export type MediaItem =
  | { type: "existing"; id: string }
  | { type: "pending"; file: File; preview: string }

export function createPendingItem(file: File): MediaItem {
  return {
    type: "pending",
    file,
    preview: URL.createObjectURL(file),
  }
}

export function getExistingIds(items: MediaItem[]): string[] {
  return items.filter((i) => i.type === "existing").map((i) => i.id)
}

export function getPendingFiles(items: MediaItem[]): File[] {
  return items.filter((i) => i.type === "pending").map((i) => i.file)
}

export function cleanupPreviews(items: MediaItem[]) {
  for (const item of items) {
    if (item.type === "pending") {
      URL.revokeObjectURL(item.preview)
    }
  }
}
