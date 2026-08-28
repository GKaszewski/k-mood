import { useState } from "react"
import { Link } from "@tanstack/react-router"
import { ChevronDown, ChevronUp, WifiOff } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { MoodPicker } from "@/components/mood/mood-picker"
import { LoggedAtPicker } from "@/components/entry/logged-at-picker"
import { ActivityPicker } from "@/components/activity/activity-picker"
import { EntryDetails } from "./entry-details"
import { DraftRestoredNotice } from "./draft-restored-notice"
import { useActivities } from "@/hooks/use-activities"
import { useOnline } from "@/hooks/use-online"
import { hasExtras } from "@/lib/entry-draft"
import { fromLoggedAt, toLoggedAt } from "@/lib/logged-at"
import type { DraftControls } from "@/hooks/use-entry-draft"

interface EntryFormProps {
  controls: DraftControls
  title: string
  submitLabel: string
  isSaving: boolean
  onSubmit: () => void
}

export function EntryForm({
  controls,
  title,
  submitLabel,
  isSaving,
  onSubmit,
}: EntryFormProps) {
  const { draft, isLoading, wasRestored, set, toggleActivity, discard } =
    controls
  const [detailsOpen, setDetailsOpen] = useState(() => hasExtras(draft))
  const { data: activities } = useActivities()
  const online = useOnline()

  const showingDetails = detailsOpen || hasExtras(draft)
  const inUse = activities?.filter((activity) => !activity.archived) ?? []

  if (isLoading) {
    return (
      <div className="flex flex-col gap-6">
        <Skeleton className="h-8 w-48 self-center" />
        <Skeleton className="h-24" />
        <Skeleton className="h-32" />
      </div>
    )
  }

  return (
    <form
      className="flex flex-col gap-6"
      onSubmit={(event) => {
        event.preventDefault()
        onSubmit()
      }}
    >
      <h1 className="text-center text-xl font-bold">{title}</h1>

      {wasRestored && <DraftRestoredNotice onDiscard={discard} />}

      <MoodPicker
        value={draft.mood ?? undefined}
        onChange={(mood) => set("mood", mood)}
        size="lg"
      />

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Logged</CardTitle>
        </CardHeader>
        <CardContent>
          <LoggedAtPicker
            value={fromLoggedAt(draft.loggedAt)}
            onChange={(moment) => set("loggedAt", toLoggedAt(moment))}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Activities</CardTitle>
        </CardHeader>
        <CardContent>
          {inUse.length > 0 ? (
            <ActivityPicker
              activities={inUse}
              selected={draft.activityIds}
              onToggle={toggleActivity}
            />
          ) : (
            <p className="text-sm text-muted-foreground">
              No activities yet.{" "}
              <Link to="/activities" className="text-primary underline">
                Add some
              </Link>{" "}
              to tag what you were doing.
            </p>
          )}
        </CardContent>
      </Card>

      {showingDetails ? (
        <EntryDetails {...controls} />
      ) : (
        <Button
          type="button"
          variant="outline"
          aria-expanded={false}
          onClick={() => setDetailsOpen(true)}
        >
          <ChevronDown className="mr-2 h-4 w-4" aria-hidden="true" />
          Add a note, photo, or more
        </Button>
      )}

      {showingDetails && !hasExtras(draft) && (
        <Button
          type="button"
          variant="ghost"
          size="sm"
          aria-expanded={true}
          onClick={() => setDetailsOpen(false)}
        >
          <ChevronUp className="mr-2 h-4 w-4" aria-hidden="true" />
          Hide extras
        </Button>
      )}

      {!online && (
        <p className="flex items-center justify-center gap-2 text-xs text-muted-foreground">
          <WifiOff className="h-3 w-3" aria-hidden="true" />
          Offline — this will send when you are back
        </p>
      )}

      <Button
        type="submit"
        size="lg"
        disabled={draft.mood === null || isSaving}
        className="w-full"
      >
        {isSaving ? "Saving…" : submitLabel}
      </Button>
    </form>
  )
}
