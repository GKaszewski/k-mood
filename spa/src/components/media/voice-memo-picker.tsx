import { useRef } from "react"
import { FileAudio, Mic, Square } from "lucide-react"
import { Button } from "@/components/ui/button"
import { AudioPlayer } from "./audio-player"
import { media } from "@/api/client"
import { useVoiceRecorder, formatDuration } from "@/hooks/use-voice-recorder"
import type { MediaItem } from "@/lib/media"
import { createPendingItem } from "@/lib/media"

interface VoiceMemoPickerProps {
  items: MediaItem[]
  onChange: (items: MediaItem[]) => void
}

export function VoiceMemoPicker({ items, onChange }: VoiceMemoPickerProps) {
  const fileRef = useRef<HTMLInputElement>(null)

  const addFile = (file: File) => {
    onChange([...items, createPendingItem(file)])
  }

  const { recording, duration, start, stop } = useVoiceRecorder(addFile)

  const handleFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return
    addFile(file)
    e.target.value = ""
  }

  const remove = (index: number) => {
    onChange(items.filter((_, i) => i !== index))
  }

  return (
    <div className="flex flex-col gap-3">
      {items.map((item, i) => (
        <AudioPlayer
          key={i}
          src={
            item.type === "existing"
              ? media.voiceMemoUrl(item.id)
              : item.preview
          }
          onRemove={() => remove(i)}
        />
      ))}

      {recording && (
        <div className="flex items-center gap-3 rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3">
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
            <Square className="mr-2 h-4 w-4" />
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
            <Mic className="mr-2 h-4 w-4" />
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
          <FileAudio className="mr-2 h-4 w-4" />
          Upload
        </Button>
      </div>
    </div>
  )
}
