import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { MoodPicker } from "@/components/mood/mood-picker"
import { LoggedAtPicker } from "@/components/entry/logged-at-picker"
import { ActivityPicker } from "@/components/activity/activity-picker"
import { PhotoPicker } from "@/components/media/photo-picker"
import { VoiceMemoPicker } from "@/components/media/voice-memo-picker"
import { useActivities } from "@/hooks/use-activities"
import { useNowPlaying } from "@/hooks/use-providers"
import { media as mediaApi } from "@/api/client"
import { toast } from "@/components/ui/toast"
import type { MoodValue } from "@/lib/mood"
import type { MediaItem } from "@/lib/media"
import { getExistingIds, getPendingFiles } from "@/lib/media"
import { buildDimensions } from "@/lib/dimensions"
import { fromLoggedAt, toLoggedAt } from "@/lib/logged-at"
import type { DimensionPayload } from "@/api/schema"

export interface EntryFormData {
  mood: number
  loggedAt: string
  dimensions?: DimensionPayload[]
}

interface EntryFormProps {
  title: string
  submitLabel: string
  isPending: boolean
  onSubmit: (data: EntryFormData) => void
  initial?: {
    mood?: number
    loggedAt?: string
    content?: string
    activities?: string[]
    photos?: string[]
    voiceMemos?: string[]
    location?: { latitude: number; longitude: number } | null
    song?: { title: string; artist: string } | null
  }
}

export function EntryForm({
  title,
  submitLabel,
  isPending,
  onSubmit,
  initial,
}: EntryFormProps) {
  const [mood, setMood] = useState<MoodValue | undefined>(
    initial?.mood as MoodValue | undefined
  )
  const [loggedAt, setLoggedAt] = useState<Date>(
    initial?.loggedAt ? fromLoggedAt(initial.loggedAt) : new Date()
  )
  const [content, setContent] = useState(initial?.content ?? "")
  const [selectedActivities, setSelectedActivities] = useState<string[]>(
    initial?.activities ?? []
  )
  const [photoItems, setPhotoItems] = useState<MediaItem[]>(
    (initial?.photos ?? []).map((id) => ({ type: "existing" as const, id }))
  )
  const [voiceMemoItems, setVoiceMemoItems] = useState<MediaItem[]>(
    (initial?.voiceMemos ?? []).map((id) => ({ type: "existing" as const, id }))
  )
  const [songTitle, setSongTitle] = useState(initial?.song?.title ?? "")
  const [songArtist, setSongArtist] = useState(initial?.song?.artist ?? "")
  const [location, setLocation] = useState<{
    latitude: number
    longitude: number
  } | null>(initial?.location ?? null)
  const [locating, setLocating] = useState(false)
  const nowPlaying = useNowPlaying()

  const fillFromNowPlaying = () => {
    nowPlaying.mutate(undefined, {
      onSuccess: (playing) => {
        if (playing?.kind !== "song") {
          toast.add({ title: "Nothing is playing right now", type: "info" })
          return
        }
        setSongTitle(playing.title)
        setSongArtist(playing.artist)
      },
      onError: () =>
        toast.add({
          title: "Could not reach your music server",
          type: "error",
        }),
    })
  }
  const [uploading, setUploading] = useState(false)

  const captureLocation = () => {
    if (!("geolocation" in navigator)) {
      toast.add({
        title: "Location is unavailable on this device",
        type: "error",
      })
      return
    }
    setLocating(true)
    navigator.geolocation.getCurrentPosition(
      (position) => {
        setLocation({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        })
        setLocating(false)
      },
      () => {
        toast.add({ title: "Could not read your location", type: "error" })
        setLocating(false)
      }
    )
  }
  const { data: activities } = useActivities()

  const toggleActivity = (id: string) => {
    setSelectedActivities((prev) =>
      prev.includes(id) ? prev.filter((a) => a !== id) : [...prev, id]
    )
  }

  const handleSubmit = async () => {
    if (!mood) return

    setUploading(true)
    try {
      const photoIds = [...getExistingIds(photoItems)]
      for (const file of getPendingFiles(photoItems)) {
        const result = await mediaApi.uploadPhoto(file)
        photoIds.push(result.id)
      }

      const voiceMemoIds = [...getExistingIds(voiceMemoItems)]
      for (const file of getPendingFiles(voiceMemoItems)) {
        const result = await mediaApi.uploadVoiceMemo(file)
        voiceMemoIds.push(result.id)
      }

      onSubmit({
        mood,
        loggedAt: toLoggedAt(loggedAt),
        dimensions: buildDimensions({
          content,
          activityIds: selectedActivities,
          photoIds,
          voiceMemoIds,
          location,
          song: { title: songTitle, artist: songArtist },
        }),
      })
    } catch {
      toast.add({ title: "Failed to upload media", type: "error" })
    } finally {
      setUploading(false)
    }
  }

  const busy = isPending || uploading

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-center text-xl font-bold">{title}</h1>

      <MoodPicker value={mood} onChange={setMood} size="lg" />

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Logged</CardTitle>
        </CardHeader>
        <CardContent>
          <LoggedAtPicker value={loggedAt} onChange={setLoggedAt} />
        </CardContent>
      </Card>

      {activities?.length ? (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">Activities</CardTitle>
          </CardHeader>
          <CardContent>
            <ActivityPicker
              activities={activities}
              selected={selectedActivities}
              onToggle={toggleActivity}
            />
          </CardContent>
        </Card>
      ) : null}

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Note</CardTitle>
        </CardHeader>
        <CardContent>
          <Textarea
            placeholder="What's on your mind?"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            rows={3}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Song</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          <Input
            placeholder="Title"
            value={songTitle}
            onChange={(e) => setSongTitle(e.target.value)}
          />
          <Input
            placeholder="Artist"
            value={songArtist}
            onChange={(e) => setSongArtist(e.target.value)}
          />
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={fillFromNowPlaying}
            disabled={nowPlaying.isPending}
          >
            {nowPlaying.isPending ? "Checking…" : "What's playing?"}
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Location</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center gap-3">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={captureLocation}
            disabled={locating}
          >
            {locating
              ? "Locating…"
              : location
                ? "Update location"
                : "Add location"}
          </Button>
          {location && (
            <span className="text-xs text-muted-foreground">
              {location.latitude.toFixed(4)}, {location.longitude.toFixed(4)}
            </span>
          )}
          {location && (
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onClick={() => setLocation(null)}
            >
              Remove
            </Button>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Photos</CardTitle>
        </CardHeader>
        <CardContent>
          <PhotoPicker items={photoItems} onChange={setPhotoItems} />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Voice memos</CardTitle>
        </CardHeader>
        <CardContent>
          <VoiceMemoPicker
            items={voiceMemoItems}
            onChange={setVoiceMemoItems}
          />
        </CardContent>
      </Card>

      <Button
        size="lg"
        onClick={handleSubmit}
        disabled={!mood || busy}
        className="w-full"
      >
        {uploading
          ? "Uploading media..."
          : isPending
            ? "Saving..."
            : submitLabel}
      </Button>
    </div>
  )
}
