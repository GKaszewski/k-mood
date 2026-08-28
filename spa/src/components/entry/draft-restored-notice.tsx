import { Button } from "@/components/ui/button"

/**
 * Says plainly that what is on screen was picked up from an abandoned attempt,
 * rather than letting it look like a fresh form that mysteriously has content.
 */
export function DraftRestoredNotice({ onDiscard }: { onDiscard: () => void }) {
  return (
    <div
      role="status"
      className="flex items-center justify-between gap-3 rounded-lg border border-primary/40 bg-primary/10 px-3 py-2"
    >
      <span className="text-xs">We kept what you started earlier.</span>
      <Button type="button" variant="ghost" size="sm" onClick={onDiscard}>
        Start over
      </Button>
    </div>
  )
}
