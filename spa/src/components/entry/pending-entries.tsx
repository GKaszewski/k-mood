import { format } from "date-fns"
import { AlertTriangle, Clock } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { MoodBadge } from "@/components/mood/mood-badge"
import {
  useDiscardQueued,
  useQueuedEntries,
  useRetryQueued,
} from "@/hooks/use-outbox"
import { isStuck } from "@/lib/outbox-sync"
import { contentPreview } from "@/lib/entry-preview"
import type { QueuedEntry } from "@/lib/store/outbox"

/**
 * Entries written on this device that the server has not taken yet. They are
 * shown with the journal so that logging offline still looks like it worked.
 */
export function PendingEntries() {
  const { data: queued = [] } = useQueuedEntries()

  if (!queued.length) return null

  return (
    <section className="flex flex-col gap-2">
      <h2 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
        Not sent yet
      </h2>
      {queued.map((entry) => (
        <PendingEntryCard key={entry.id} queued={entry} />
      ))}
    </section>
  )
}

function PendingEntryCard({ queued }: { queued: QueuedEntry }) {
  const retry = useRetryQueued()
  const discard = useDiscardQueued()
  const stuck = isStuck(queued)
  const note = contentPreview(queued.draft.content)

  return (
    <Card className="border-dashed">
      <CardContent className="flex items-start gap-3 p-4">
        {queued.draft.mood !== null && <MoodBadge value={queued.draft.mood} />}

        <div className="min-w-0 flex-1">
          <div className="flex items-center justify-between gap-2">
            <span className="flex items-center gap-1 text-xs text-muted-foreground">
              {stuck ? (
                <AlertTriangle
                  className="h-3 w-3 text-destructive"
                  aria-hidden="true"
                />
              ) : (
                <Clock className="h-3 w-3" aria-hidden="true" />
              )}
              {stuck ? "The server would not take this" : "Waiting to send"}
            </span>
            <span className="text-xs text-muted-foreground">
              {format(new Date(queued.draft.loggedAt), "MMM d, h:mm a")}
            </span>
          </div>

          {note && (
            <p className="mt-1 line-clamp-2 text-sm text-muted-foreground">
              {note}
            </p>
          )}

          {stuck && (
            <>
              {queued.lastError && (
                <p className="mt-1 text-xs text-destructive">
                  {queued.lastError}
                </p>
              )}
              <div className="mt-2 flex gap-2">
                <Button
                  size="sm"
                  variant="outline"
                  disabled={retry.isPending}
                  onClick={() => retry.mutate(queued)}
                >
                  {retry.isPending ? "Retrying…" : "Try again"}
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  className="text-destructive"
                  disabled={discard.isPending}
                  onClick={() => discard.mutate(queued.id)}
                >
                  Discard
                </Button>
              </div>
            </>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
