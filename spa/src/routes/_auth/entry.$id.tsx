import {
  activitiesOf,
  contentOf,
  locationOf,
  photosOf,
  songOf,
  voiceMemosOf,
} from "@/lib/dimensions"
import { createFileRoute, Link, useNavigate } from "@tanstack/react-router"
import { format } from "date-fns"
import { ArrowLeft, Pencil, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { EntryWeather } from "@/components/entry/entry-weather"
import { Badge } from "@/components/ui/badge"
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
import { useEntry, useDeleteEntry } from "@/hooks/use-entries"
import { useActivities } from "@/hooks/use-activities"
import { media } from "@/api/client"

export const Route = createFileRoute("/_auth/entry/$id")({
  component: EntryDetailPage,
})

function EntryDetailPage() {
  const { id } = Route.useParams()
  const navigate = useNavigate()
  const { data: entry, isLoading } = useEntry(id)
  const { data: activities } = useActivities()
  const deleteEntry = useDeleteEntry()

  const handleDelete = () => {
    deleteEntry.mutate(id, {
      onSuccess: () => navigate({ to: "/" }),
    })
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

  const date = new Date(entry.loggedAt)

  return (
    <div className="flex flex-col gap-4 py-6">
      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate({ to: "/" })}
        >
          <ArrowLeft className="h-5 w-5" />
        </Button>

        <div className="flex items-center gap-1">
          <Link to="/edit/$id" params={{ id }}>
            <Button variant="ghost" size="icon">
              <Pencil className="h-5 w-5" />
            </Button>
          </Link>

          <AlertDialog>
            <AlertDialogTrigger className="inline-flex h-9 w-9 items-center justify-center rounded-md text-destructive hover:bg-muted">
              <Trash2 className="h-5 w-5" />
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
                <AlertDialogAction onClick={handleDelete}>
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
            {format(date, "EEEE, MMMM d, yyyy")}
          </p>
          <p className="text-sm text-muted-foreground">
            {format(date, "h:mm a")}
          </p>
        </CardContent>
      </Card>

      {contentOf(entry) && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Note</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm whitespace-pre-wrap">{contentOf(entry)}</p>
          </CardContent>
        </Card>
      )}

      {songOf(entry) && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Listening to</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm">
              {songOf(entry)?.title}
              <span className="text-muted-foreground">
                {" "}
                — {songOf(entry)?.artist}
              </span>
            </p>
          </CardContent>
        </Card>
      )}

      {locationOf(entry) && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Location</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-1.5">
            <p className="text-sm text-muted-foreground">
              {locationOf(entry)?.latitude.toFixed(4)},{" "}
              {locationOf(entry)?.longitude.toFixed(4)}
            </p>
            <EntryWeather entry={entry} />
          </CardContent>
        </Card>
      )}

      {activitiesOf(entry).length > 0 && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Activities</CardTitle>
          </CardHeader>
          <CardContent>
            <GroupedActivityBadges
              activityIds={activitiesOf(entry)}
              allActivities={activities ?? []}
            />
          </CardContent>
        </Card>
      )}

      {photosOf(entry).length > 0 && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Photos</CardTitle>
          </CardHeader>
          <CardContent className="grid grid-cols-2 gap-2">
            {photosOf(entry).map((photoId) => (
              <img
                key={photoId}
                src={media.photoUrl(photoId)}
                alt=""
                className="aspect-square w-full rounded-lg object-cover"
              />
            ))}
          </CardContent>
        </Card>
      )}

      {voiceMemosOf(entry).length > 0 && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Voice memos</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            {voiceMemosOf(entry).map((memoId) => (
              <AudioPlayer key={memoId} src={media.voiceMemoUrl(memoId)} />
            ))}
          </CardContent>
        </Card>
      )}
    </div>
  )
}

function GroupedActivityBadges({
  activityIds,
  allActivities,
}: {
  activityIds: string[]
  allActivities: Array<{ id: string; name: string; category?: string | null }>
}) {
  const lookup = new Map(allActivities.map((a) => [a.id, a]))
  const grouped = new Map<string, string[]>()

  for (const id of activityIds) {
    const activity = lookup.get(id)
    const cat = activity?.category ?? "Other"
    if (!grouped.has(cat)) grouped.set(cat, [])
    grouped.get(cat)!.push(activity?.name ?? id.slice(0, 8))
  }

  const entries = Array.from(grouped.entries()).sort(([a], [b]) => {
    if (a === "Other") return 1
    if (b === "Other") return -1
    return a.localeCompare(b)
  })

  return (
    <div className="flex flex-col gap-3">
      {entries.map(([category, names]) => (
        <div key={category}>
          <p className="mb-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {category}
          </p>
          <div className="flex flex-wrap gap-2">
            {names.map((name) => (
              <Badge key={name} variant="outline" className="text-xs">
                {name}
              </Badge>
            ))}
          </div>
        </div>
      ))}
    </div>
  )
}
