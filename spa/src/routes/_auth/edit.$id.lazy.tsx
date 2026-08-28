import { createLazyFileRoute, useRouter } from "@tanstack/react-router"
import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { EntryForm } from "@/components/entry/entry-form"
import { EntryMissing } from "@/components/entry/entry-missing"
import { useEntry } from "@/hooks/use-entries"
import { useEntryDraft } from "@/hooks/use-entry-draft"
import { useSaveEntryEdit } from "@/hooks/use-save-entry"
import { draftOf } from "@/lib/entry-draft"
import type { EntryResponse } from "@/api/schema"

export const Route = createLazyFileRoute("/_auth/edit/$id")({
  component: EditEntryPage,
})

function EditEntryPage() {
  const { id } = Route.useParams()
  const { data: entry, isLoading } = useEntry(id)

  if (isLoading) {
    return (
      <div className="flex flex-col gap-4 py-6">
        <Skeleton className="h-8 w-32" />
        <Skeleton className="h-40" />
      </div>
    )
  }

  if (!entry) return <EntryMissing />

  return <EditEntryForm entry={entry} />
}

function EditEntryForm({ entry }: { entry: EntryResponse }) {
  const router = useRouter()
  const controls = useEntryDraft({ initial: draftOf(entry), persist: false })
  const save = useSaveEntryEdit(entry.id)

  return (
    <div className="py-6">
      <div className="mb-4 flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon"
          aria-label="Go back"
          onClick={() => router.history.back()}
        >
          <ArrowLeft className="h-5 w-5" aria-hidden="true" />
        </Button>
      </div>

      <EntryForm
        controls={controls}
        title="Edit entry"
        submitLabel="Save changes"
        isSaving={save.isPending}
        onSubmit={() =>
          save.mutate(controls.draft, {
            onSuccess: () => router.history.back(),
          })
        }
      />
    </div>
  )
}
