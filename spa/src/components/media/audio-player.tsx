import { useCallback, useEffect, useRef, useState } from "react"
import { Pause, Play, X } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Slider } from "@/components/ui/slider"
import { formatDuration } from "@/lib/duration"

interface AudioPlayerProps {
  src: string
  label?: string
  onRemove?: () => void
}

export function AudioPlayer({
  src,
  label = "Voice memo",
  onRemove,
}: AudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement>(null)
  const [playing, setPlaying] = useState(false)
  const [duration, setDuration] = useState(0)
  const [currentTime, setCurrentTime] = useState(0)

  useEffect(() => {
    const audio = audioRef.current
    if (!audio) return

    const onLoaded = () => setDuration(audio.duration)
    const onTime = () => setCurrentTime(audio.currentTime)
    const onEnded = () => setPlaying(false)
    const onPlay = () => setPlaying(true)
    const onPause = () => setPlaying(false)

    audio.addEventListener("loadedmetadata", onLoaded)
    audio.addEventListener("timeupdate", onTime)
    audio.addEventListener("ended", onEnded)
    audio.addEventListener("play", onPlay)
    audio.addEventListener("pause", onPause)

    return () => {
      audio.removeEventListener("loadedmetadata", onLoaded)
      audio.removeEventListener("timeupdate", onTime)
      audio.removeEventListener("ended", onEnded)
      audio.removeEventListener("play", onPlay)
      audio.removeEventListener("pause", onPause)
    }
  }, [])

  const toggle = useCallback(() => {
    const audio = audioRef.current
    if (!audio) return

    if (audio.paused) {
      audio.play().catch(() => setPlaying(false))
    } else {
      audio.pause()
    }
  }, [])

  const seek = useCallback((value: number | readonly number[]) => {
    const audio = audioRef.current
    if (!audio) return

    const moment = Array.isArray(value) ? value[0] : (value as number)
    audio.currentTime = moment
    setCurrentTime(moment)
  }, [])

  return (
    <div className="flex items-center gap-2 rounded-lg border bg-muted/30 px-3 py-2">
      <audio ref={audioRef} src={src} preload="metadata" />

      <Button
        type="button"
        variant="ghost"
        size="icon"
        aria-label={`${playing ? "Pause" : "Play"} ${label}`}
        className="h-8 w-8 shrink-0"
        onClick={toggle}
      >
        {playing ? (
          <Pause className="h-4 w-4" aria-hidden="true" />
        ) : (
          <Play className="h-4 w-4" aria-hidden="true" />
        )}
      </Button>

      <Slider
        value={[currentTime]}
        max={duration || 1}
        step={0.1}
        onValueChange={seek}
        aria-label={`Seek within ${label}`}
        className="flex-1"
      />

      <span className="shrink-0 text-xs text-muted-foreground tabular-nums">
        {formatDuration(currentTime)}/{formatDuration(duration)}
      </span>

      {onRemove && (
        <Button
          type="button"
          variant="ghost"
          size="icon"
          aria-label={`Remove ${label}`}
          className="h-6 w-6 shrink-0 text-destructive"
          onClick={onRemove}
        >
          <X className="h-3 w-3" aria-hidden="true" />
        </Button>
      )}
    </div>
  )
}
