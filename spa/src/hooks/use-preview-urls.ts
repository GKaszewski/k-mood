import { useEffect, useMemo } from "react"
import type { MediaItem } from "@/lib/media"

/**
 * Object URLs for media picked on this device, revoked as soon as the set they
 * belong to is replaced. Without the revoke every discarded photo stays in
 * memory for the life of the tab.
 */
export function usePreviewUrls(items: MediaItem[]): Map<string, string> {
  const urls = useMemo(() => {
    const created = new Map<string, string>()

    for (const item of items) {
      if (item.kind === "pending") {
        created.set(item.localId, URL.createObjectURL(item.file))
      }
    }

    return created
  }, [items])

  useEffect(() => {
    return () => {
      for (const url of urls.values()) URL.revokeObjectURL(url)
    }
  }, [urls])

  return urls
}
