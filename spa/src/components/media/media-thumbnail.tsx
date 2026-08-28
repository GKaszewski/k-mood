import { X } from "lucide-react"

interface MediaThumbnailProps {
  src: string
  alt: string
  onRemove: () => void
}

export function MediaThumbnail({ src, alt, onRemove }: MediaThumbnailProps) {
  return (
    <div className="relative h-20 w-20">
      <img
        src={src}
        alt={alt}
        className="h-full w-full rounded-lg object-cover"
      />
      <button
        type="button"
        aria-label={`Remove ${alt}`}
        onClick={onRemove}
        className="text-destructive-foreground absolute -top-1 -right-1 rounded-full bg-destructive p-0.5"
      >
        <X className="h-3 w-3" aria-hidden="true" />
      </button>
    </div>
  )
}
