import { CloudUpload, WifiOff } from "lucide-react"
import { useOnline } from "@/hooks/use-online"
import { useQueuedEntries } from "@/hooks/use-outbox"
import { isStuck } from "@/lib/outbox-sync"

/**
 * Says where unsent work stands. Silent when there is nothing to say — a
 * permanent banner would be noise on every screen.
 */
export function ConnectionBanner() {
  const online = useOnline()
  const { data: queued = [] } = useQueuedEntries()

  const stuck = queued.filter(isStuck).length
  const waiting = queued.length - stuck

  if (online && queued.length === 0) return null

  return (
    <div
      role="status"
      className="flex items-center gap-2 rounded-lg border border-border bg-muted/60 px-3 py-2 text-xs"
    >
      {online ? (
        <CloudUpload className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
      ) : (
        <WifiOff className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
      )}
      <span>{summarise({ online, waiting, stuck })}</span>
    </div>
  )
}

function plural(count: number): string {
  return count === 1 ? "entry" : "entries"
}

function summarise({
  online,
  waiting,
  stuck,
}: {
  online: boolean
  waiting: number
  stuck: number
}): string {
  if (!online) {
    return waiting + stuck > 0
      ? `Offline. ${waiting + stuck} ${plural(waiting + stuck)} waiting to send.`
      : "Offline. Anything you log is kept here and sent when you are back."
  }

  if (stuck > 0 && waiting > 0) {
    return `Sending ${waiting} ${plural(waiting)}. ${stuck} could not be sent.`
  }

  if (stuck > 0) {
    return `${stuck} ${plural(stuck)} could not be sent — review below.`
  }

  return `Sending ${waiting} ${plural(waiting)}…`
}
