import { useRef } from "react"
import { FileAudio, Mic, Square } from "lucide-react"
import { Button } from "@/components/ui/button"
import { AudioPlayer } from "./audio-player"
import { media } from "@/api/client"
import { usePreviewUrls } from "@/hooks/use-preview-urls"
import { useVoiceRecorder } from "@/hooks/use-voice-recorder"
import { formatDuration } from "@/lib/duration"
import { keyOf, newPendingItem } from "@/lib/media"
import type { MediaItem } from "@/lib/media"

interface VoiceMemoPickerProps {
  items: MediaItem[]
  onChange: (items: MediaItem[]) => void
}

export function VoiceMemoPicker({ items, onChange }: VoiceMemoPickerProps) {
  const fileRef = useRef<HTMLInputElement>(null)
  const previews = usePreviewUrls(items)

  const add = (file: File) => onChange([...items, newPendingItem(file)])

  const { recording, duration, start, stop } = useVoiceRecorder(add)

  const handleFile = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) add(file)
    event.target.value = ""
  }

  const removeAt = (index: number) => {
    onChange(items.filter((_, position) => position !== index))
  }

  return (
    <div className="flex flex-col gap-3">
      {items.map((item, index) => (
        <AudioPlayer
          key={keyOf(item)}
          src={
            item.kind === "existing"
              ? media.voiceMemoUrl(item.id)
              : (previews.get(item.localId) ?? "")
          }
          label={`Voice memo ${index + 1}`}
          onRemove={() => removeAt(index)}
        />
      ))}

      {recording && (
        <div
          role="status"
          className="flex items-center gap-3 rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3"
        >
          <span className="h-2.5 w-2.5 animate-pulse rounded-full bg-destructive" />
          <span className="text-sm font-medium">Recording</span>
          <span className="ml-auto font-mono text-sm">
            {formatDuration(duration)}
          </span>
        </div>
      )}

      <input
        ref={fileRef}
        type="file"
        accept="audio/*"
        onChange={handleFile}
        className="hidden"
      />

      <div className="flex gap-2">
        {recording ? (
          <Button
            type="button"
            variant="destructive"
            size="sm"
            className="flex-1"
            onClick={stop}
          >
            <Square className="mr-2 h-4 w-4" aria-hidden="true" />
            Stop
          </Button>
        ) : (
          <Button
            type="button"
            variant="outline"
            size="sm"
            className="flex-1"
            onClick={start}
          >
            <Mic className="mr-2 h-4 w-4" aria-hidden="true" />
            Record
          </Button>
        )}
        <Button
          type="button"
          variant="outline"
          size="sm"
          className="flex-1"
          onClick={() => fileRef.current?.click()}
          disabled={recording}
        >
          <FileAudio className="mr-2 h-4 w-4" aria-hidden="true" />
          Upload
        </Button>
      </div>
    </div>
  )
}
