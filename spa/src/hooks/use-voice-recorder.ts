import { useState, useRef, useCallback, useEffect } from "react"
import { toast } from "@/components/ui/toast"

export function useVoiceRecorder(onRecorded: (file: File) => void) {
  const [recording, setRecording] = useState(false)
  const [duration, setDuration] = useState(0)
  const recorderRef = useRef<MediaRecorder | null>(null)
  const chunksRef = useRef<Blob[]>([])
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null)

  useEffect(() => {
    return () => {
      if (timerRef.current) clearInterval(timerRef.current)
    }
  }, [])

  const start = useCallback(async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true })
      const recorder = new MediaRecorder(stream)
      chunksRef.current = []
      setDuration(0)

      timerRef.current = setInterval(() => {
        setDuration((d) => d + 1)
      }, 1000)

      recorder.ondataavailable = (e) => {
        if (e.data.size > 0) chunksRef.current.push(e.data)
      }

      recorder.onstop = () => {
        stream.getTracks().forEach((t) => t.stop())
        if (timerRef.current) {
          clearInterval(timerRef.current)
          timerRef.current = null
        }
        const blob = new Blob(chunksRef.current, { type: "audio/webm" })
        onRecorded(new File([blob], "recording.webm", { type: "audio/webm" }))
        setDuration(0)
      }

      recorder.start()
      recorderRef.current = recorder
      setRecording(true)
    } catch {
      toast.add({ title: "Microphone access denied", type: "error" })
    }
  }, [onRecorded])

  const stop = useCallback(() => {
    recorderRef.current?.stop()
    recorderRef.current = null
    setRecording(false)
  }, [])

  return { recording, duration, start, stop }
}

export function formatDuration(seconds: number): string {
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${m}:${s.toString().padStart(2, "0")}`
}
