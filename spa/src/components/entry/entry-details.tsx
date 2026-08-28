import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { PhotoPicker } from "@/components/media/photo-picker"
import { VoiceMemoPicker } from "@/components/media/voice-memo-picker"
import { SongField } from "./fields/song-field"
import { LocationField } from "./fields/location-field"
import type { DraftControls } from "@/hooks/use-entry-draft"

const NOTE_ROWS = 3

/**
 * Everything beyond the mood, the moment and the activities. Folded away by
 * default so the common case — a face and a tap — is not buried under it.
 */
export function EntryDetails({ draft, set }: DraftControls) {
  return (
    <div className="flex flex-col gap-4">
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">
            <Label htmlFor="entry-note">Note</Label>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <Textarea
            id="entry-note"
            placeholder="What's on your mind?"
            value={draft.content}
            onChange={(event) => set("content", event.target.value)}
            rows={NOTE_ROWS}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Song</CardTitle>
        </CardHeader>
        <CardContent>
          <SongField
            value={draft.song}
            onChange={(song) => set("song", song)}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Location</CardTitle>
        </CardHeader>
        <CardContent>
          <LocationField
            value={draft.location}
            onChange={(location) => set("location", location)}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Photos</CardTitle>
        </CardHeader>
        <CardContent>
          <PhotoPicker
            items={draft.photos}
            onChange={(photos) => set("photos", photos)}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">Voice memos</CardTitle>
        </CardHeader>
        <CardContent>
          <VoiceMemoPicker
            items={draft.voiceMemos}
            onChange={(voiceMemos) => set("voiceMemos", voiceMemos)}
          />
        </CardContent>
      </Card>
    </div>
  )
}
