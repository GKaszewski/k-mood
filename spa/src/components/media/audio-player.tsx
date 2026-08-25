import { useRef, useState, useEffect, useCallback } from "react"
import { Pause, Play, X } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Slider } from "@/components/ui/slider"

interface AudioPlayerProps {
  src: string
  onRemove?: () => void
}

export function AudioPlayer({ src, onRemove }: AudioPlayerProps) {
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

    audio.addEventListener("loadedmetadata", onLoaded)
    audio.addEventListener("timeupdate", onTime)
    audio.addEventListener("ended", onEnded)

    return () => {
      audio.removeEventListener("loadedmetadata", onLoaded)
      audio.removeEventListener("timeupdate", onTime)
      audio.removeEventListener("ended", onEnded)
    }
  }, [])

  const toggle = useCallback(() => {
    const audio = audioRef.current
    if (!audio) return
    if (playing) {
      audio.pause()
    } else {
      audio.play()
    }
    setPlaying(!playing)
  }, [playing])

  const seek = useCallback((value: number | readonly number[]) => {
    const audio = audioRef.current
    if (!audio) return
    const v = Array.isArray(value) ? value[0] : value
    audio.currentTime = v
    setCurrentTime(v)
  }, [])

  return (
    <div className="flex items-center gap-2 rounded-lg border bg-muted/30 px-3 py-2">
      <audio ref={audioRef} src={src} preload="metadata" />

      <Button
        type="button"
        variant="ghost"
        size="icon"
        className="h-8 w-8 shrink-0"
        onClick={toggle}
      >
        {playing ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
      </Button>

      <Slider
        value={[currentTime]}
        max={duration || 1}
        step={0.1}
        onValueChange={seek}
        className="flex-1"
      />

      <span className="shrink-0 text-xs text-muted-foreground tabular-nums">
        {fmt(currentTime)}/{fmt(duration)}
      </span>

      {onRemove && (
        <Button
          type="button"
          variant="ghost"
          size="icon"
          className="h-6 w-6 shrink-0 text-destructive"
          onClick={onRemove}
        >
          <X className="h-3 w-3" />
        </Button>
      )}
    </div>
  )
}

function fmt(seconds: number): string {
  if (!isFinite(seconds)) return "0:00"
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, "0")}`
}
