import { useCallback, useEffect, useRef, useState } from "react"
import { emptyDraft, isWorthKeeping } from "@/lib/entry-draft"
import type { EntryDraft } from "@/lib/entry-draft"
import { clearDraft, loadDraft, saveDraft } from "@/lib/store/draft-store"

const SAVE_AFTER_IDLE_FOR = 400

export interface DraftControls {
  draft: EntryDraft
  isLoading: boolean
  /** True when what is on screen came back from a previous, abandoned session. */
  wasRestored: boolean
  set: <TKey extends keyof EntryDraft>(
    key: TKey,
    value: EntryDraft[TKey]
  ) => void
  toggleActivity: (activityId: string) => void
  discard: () => void
}

interface Options {
  /** A draft to start from. Editing an existing entry supplies one. */
  initial?: EntryDraft
  /** Whether to keep this draft across visits. Only the new entry does. */
  persist: boolean
}

export function useEntryDraft({ initial, persist }: Options): DraftControls {
  const [draft, setDraft] = useState<EntryDraft>(initial ?? emptyDraft)
  const [isLoading, setIsLoading] = useState(persist)
  const [wasRestored, setWasRestored] = useState(false)
  const saveTimer = useRef<ReturnType<typeof setTimeout> | null>(null)

  useEffect(() => {
    if (!persist) return

    let stillMounted = true

    loadDraft().then((stored) => {
      if (!stillMounted) return

      if (stored && isWorthKeeping(stored)) {
        setDraft(stored)
        setWasRestored(true)
      }
      setIsLoading(false)
    })

    return () => {
      stillMounted = false
    }
  }, [persist])

  useEffect(() => {
    if (!persist || isLoading) return

    if (saveTimer.current) clearTimeout(saveTimer.current)
    saveTimer.current = setTimeout(() => {
      if (isWorthKeeping(draft)) {
        saveDraft(draft)
      } else {
        clearDraft()
      }
    }, SAVE_AFTER_IDLE_FOR)

    return () => {
      if (saveTimer.current) clearTimeout(saveTimer.current)
    }
  }, [draft, persist, isLoading])

  const set = useCallback(
    <TKey extends keyof EntryDraft>(key: TKey, value: EntryDraft[TKey]) => {
      setDraft((current) => ({ ...current, [key]: value }))
      setWasRestored(false)
    },
    []
  )

  const toggleActivity = useCallback((activityId: string) => {
    setDraft((current) => ({
      ...current,
      activityIds: current.activityIds.includes(activityId)
        ? current.activityIds.filter((id) => id !== activityId)
        : [...current.activityIds, activityId],
    }))
    setWasRestored(false)
  }, [])

  const discard = useCallback(() => {
    setDraft(emptyDraft())
    setWasRestored(false)
    if (persist) clearDraft()
  }, [persist])

  return { draft, isLoading, wasRestored, set, toggleActivity, discard }
}
