import { createFileRoute, useNavigate } from "@tanstack/react-router"
import { EntryForm, type EntryFormData } from "@/components/entry/entry-form"
import { useCreateEntry } from "@/hooks/use-entries"

export const Route = createFileRoute("/_auth/add")({ component: AddEntryPage })

function AddEntryPage() {
  const createEntry = useCreateEntry()
  const navigate = useNavigate()

  const handleSubmit = (data: EntryFormData) => {
    createEntry.mutate(data, {
      onSuccess: () => navigate({ to: "/" }),
    })
  }

  return (
    <div className="py-6">
      <EntryForm
        title="How are you feeling?"
        submitLabel="Save entry"
        isPending={createEntry.isPending}
        onSubmit={handleSubmit}
      />
    </div>
  )
}
