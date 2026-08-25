import { createFileRoute, useNavigate } from "@tanstack/react-router"
import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { EntryForm, type EntryFormData } from "@/components/entry/entry-form"
import { useEntry } from "@/hooks/use-entries"
import { useUpdateEntry } from "@/hooks/use-update-entry"

export const Route = createFileRoute("/_auth/edit/$id")({
  component: EditEntryPage,
})

function EditEntryPage() {
  const { id } = Route.useParams()
  const navigate = useNavigate()
  const { data: entry, isLoading } = useEntry(id)
  const updateEntry = useUpdateEntry()

  const handleSubmit = (data: EntryFormData) => {
    updateEntry.mutate(
      { id, body: data },
      { onSuccess: () => navigate({ to: "/entry/$id", params: { id } }) }
    )
  }

  if (isLoading) {
    return (
      <div className="flex flex-col gap-4 py-6">
        <Skeleton className="h-8 w-32" />
        <Skeleton className="h-40" />
      </div>
    )
  }

  if (!entry) {
    return (
      <div className="flex flex-col items-center gap-4 py-12">
        <p className="text-muted-foreground">Entry not found</p>
        <Button variant="outline" onClick={() => navigate({ to: "/" })}>
          Go home
        </Button>
      </div>
    )
  }

  return (
    <div className="py-6">
      <div className="mb-4 flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate({ to: "/entry/$id", params: { id } })}
        >
          <ArrowLeft className="h-5 w-5" />
        </Button>
      </div>

      <EntryForm
        title="Edit entry"
        submitLabel="Save changes"
        isPending={updateEntry.isPending}
        onSubmit={handleSubmit}
        initial={{
          mood: entry.mood,
          content: entry.content ?? undefined,
          activities: entry.activities,
          photos: entry.photos,
          voiceMemos: entry.voiceMemos,
        }}
      />
    </div>
  )
}
