import { useEffect, useState } from "react"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"

interface PhotoLightboxProps {
  urls: string[]
  /** Which photo to show, or null when the lightbox is closed. */
  openAt: number | null
  onClose: () => void
}

export function PhotoLightbox({ urls, openAt, onClose }: PhotoLightboxProps) {
  const [index, setIndex] = useState(openAt ?? 0)

  useEffect(() => {
    if (openAt !== null) setIndex(openAt)
  }, [openAt])

  const show = (next: number) =>
    setIndex((next + urls.length) % Math.max(urls.length, 1))

  useEffect(() => {
    if (openAt === null) return

    const onKey = (event: KeyboardEvent) => {
      if (event.key === "ArrowLeft") show(index - 1)
      if (event.key === "ArrowRight") show(index + 1)
    }

    window.addEventListener("keydown", onKey)
    return () => window.removeEventListener("keydown", onKey)
  })

  const many = urls.length > 1

  return (
    <Dialog open={openAt !== null} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="max-w-[95vw] sm:max-w-2xl">
        <DialogHeader className="sr-only">
          <DialogTitle>
            Photo {index + 1} of {urls.length}
          </DialogTitle>
          <DialogDescription>A photo attached to this entry.</DialogDescription>
        </DialogHeader>

        <div className="flex items-center gap-2">
          {many && (
            <Button
              variant="ghost"
              size="icon"
              aria-label="Previous photo"
              onClick={() => show(index - 1)}
            >
              <ChevronLeft className="h-5 w-5" aria-hidden="true" />
            </Button>
          )}

          <img
            src={urls[index]}
            alt={`Photo ${index + 1} of ${urls.length}`}
            className="max-h-[75svh] w-full rounded-lg object-contain"
          />

          {many && (
            <Button
              variant="ghost"
              size="icon"
              aria-label="Next photo"
              onClick={() => show(index + 1)}
            >
              <ChevronRight className="h-5 w-5" aria-hidden="true" />
            </Button>
          )}
        </div>

        {many && (
          <p className="text-center text-xs text-muted-foreground">
            {index + 1} of {urls.length}
          </p>
        )}
      </DialogContent>
    </Dialog>
  )
}
