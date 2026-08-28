import { createLazyFileRoute, useNavigate } from "@tanstack/react-router"
import { EntryForm } from "@/components/entry/entry-form"
import { useNewEntryForm } from "@/hooks/use-new-entry-form"

export const Route = createLazyFileRoute("/_auth/add")({
  component: AddEntryPage,
})

function AddEntryPage() {
  const navigate = useNavigate()
  const { controls, submit, isSaving } = useNewEntryForm(() =>
    navigate({ to: "/" })
  )

  return (
    <div className="py-6">
      <EntryForm
        controls={controls}
        title="How are you feeling?"
        submitLabel="Save entry"
        isSaving={isSaving}
        onSubmit={submit}
      />
    </div>
  )
}
