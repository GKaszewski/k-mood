import { useEntryDraft } from "./use-entry-draft"
import { useSaveNewEntry } from "./use-save-entry"

/**
 * The new-entry form, wherever it is shown. The sheet and the full page share
 * one stored draft, so opening one after the other picks up where it left off.
 */
export function useNewEntryForm(onSaved: () => void) {
  const controls = useEntryDraft({ persist: true })
  const save = useSaveNewEntry()

  return {
    controls,
    isSaving: save.isPending,
    submit: () =>
      save.mutate(controls.draft, {
        onSuccess: () => {
          controls.discard()
          onSaved()
        },
      }),
  }
}
