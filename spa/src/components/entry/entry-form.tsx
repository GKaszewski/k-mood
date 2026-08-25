import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Textarea } from "@/components/ui/textarea"
import { MoodPicker } from "@/components/mood/mood-picker"
import { ActivityPicker } from "@/components/activity/activity-picker"
import { PhotoPicker } from "@/components/media/photo-picker"
import { VoiceMemoPicker } from "@/components/media/voice-memo-picker"
import { useActivities } from "@/hooks/use-activities"
import { media as mediaApi } from "@/api/client"
import { toast } from "@/components/ui/toast"
import type { MoodValue } from "@/lib/mood"
import type { MediaItem } from "@/lib/media"
import { getExistingIds, getPendingFiles } from "@/lib/media"

export interface EntryFormData {
  mood: number
  content?: string
  activityIds?: string[]
  photoIds?: string[]
  voiceMemoIds?: string[]
}

interface EntryFormProps {
  title: string
  submitLabel: string
  isPending: boolean
  onSubmit: (data: EntryFormData) => void
  initial?: {
    mood?: number
    content?: string
    activities?: string[]
    photos?: string[]
    voiceMemos?: string[]
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
  const [uploading, setUploading] = useState(false)
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
        content: content.trim() || undefined,
        activityIds: selectedActivities.length ? selectedActivities : undefined,
        photoIds: photoIds.length ? photoIds : undefined,
        voiceMemoIds: voiceMemoIds.length ? voiceMemoIds : undefined,
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
