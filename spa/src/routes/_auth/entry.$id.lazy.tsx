import { createLazyFileRoute, Link, useRouter } from "@tanstack/react-router"
import { buttonLook } from "@/lib/button-style"
import { format } from "date-fns"
import { ArrowLeft, Pencil, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { MoodBadge } from "@/components/mood/mood-badge"
import { AudioPlayer } from "@/components/media/audio-player"
import { EntryWeather } from "@/components/entry/entry-weather"
import { EntryPhotos } from "@/components/entry/detail/entry-photos"
import { EntryActivities } from "@/components/entry/detail/entry-activities"
import { EntryMissing } from "@/components/entry/entry-missing"
import { useEntry, useDeleteEntry } from "@/hooks/use-entries"
import { useActivities } from "@/hooks/use-activities"
import { media } from "@/api/client"
import {
  activitiesOf,
  contentOf,
  locationOf,
  photosOf,
  songOf,
  voiceMemosOf,
} from "@/lib/dimensions"

export const Route = createLazyFileRoute("/_auth/entry/$id")({
  component: EntryDetailPage,
})

const COORDINATE_PRECISION = 4

function EntryDetailPage() {
  const { id } = Route.useParams()
  const router = useRouter()
  const { data: entry, isLoading } = useEntry(id)
  const { data: activities } = useActivities()
  const deleteEntry = useDeleteEntry()

  if (isLoading) {
    return (
      <div className="flex flex-col gap-4 py-6">
        <Skeleton className="h-8 w-32" />
        <Skeleton className="h-40" />
      </div>
    )
  }

  if (!entry) return <EntryMissing />

  const loggedAt = new Date(entry.loggedAt)
  const note = contentOf(entry)
  const song = songOf(entry)
  const location = locationOf(entry)
  const voiceMemos = voiceMemosOf(entry)

  return (
    <div className="flex flex-col gap-4 py-6">
      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          size="icon"
          aria-label="Go back"
          onClick={() => router.history.back()}
        >
          <ArrowLeft className="h-5 w-5" aria-hidden="true" />
        </Button>

        <div className="flex items-center gap-1">
          <Link
            to="/edit/$id"
            params={{ id }}
            aria-label="Edit entry"
            className={buttonLook({ variant: "ghost", size: "icon" })}
          >
            <Pencil className="h-5 w-5" aria-hidden="true" />
          </Link>

          <AlertDialog>
            <AlertDialogTrigger
              aria-label="Delete entry"
              className="inline-flex h-9 w-9 items-center justify-center rounded-md text-destructive hover:bg-muted"
            >
              <Trash2 className="h-5 w-5" aria-hidden="true" />
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Delete entry?</AlertDialogTitle>
                <AlertDialogDescription>
                  This action cannot be undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={() =>
                    deleteEntry.mutate(id, {
                      onSuccess: () => router.history.back(),
                    })
                  }
                >
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </div>

      <Card>
        <CardContent className="flex flex-col items-center gap-2 py-6">
          <MoodBadge value={entry.mood} showLabel className="text-3xl" />
          <p className="text-sm text-muted-foreground">
            <time dateTime={entry.loggedAt}>
              {format(loggedAt, "EEEE, MMMM d, yyyy 'at' h:mm a")}
            </time>
          </p>
        </CardContent>
      </Card>

      {note && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Note</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm whitespace-pre-wrap">{note}</p>
          </CardContent>
        </Card>
      )}

      {song && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Listening to</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm">
              {song.title}
              <span className="text-muted-foreground"> — {song.artist}</span>
            </p>
          </CardContent>
        </Card>
      )}

      {location && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Location</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-1.5">
            <p className="text-sm text-muted-foreground tabular-nums">
              {location.latitude.toFixed(COORDINATE_PRECISION)},{" "}
              {location.longitude.toFixed(COORDINATE_PRECISION)}
            </p>
            <EntryWeather entry={entry} />
          </CardContent>
        </Card>
      )}

      <EntryActivities
        activityIds={activitiesOf(entry)}
        catalogue={activities ?? []}
      />

      <EntryPhotos photoIds={photosOf(entry)} />

      {voiceMemos.length > 0 && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Voice memos</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            {voiceMemos.map((memoId, index) => (
              <AudioPlayer
                key={memoId}
                src={media.voiceMemoUrl(memoId)}
                label={`Voice memo ${index + 1}`}
              />
            ))}
          </CardContent>
        </Card>
      )}
    </div>
  )
}
