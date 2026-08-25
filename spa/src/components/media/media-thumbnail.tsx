import { X } from "lucide-react"

interface MediaThumbnailProps {
  src: string
  onRemove: () => void
}

export function MediaThumbnail({ src, onRemove }: MediaThumbnailProps) {
  return (
    <div className="relative h-20 w-20">
      <img src={src} alt="" className="h-full w-full rounded-lg object-cover" />
      <button
        type="button"
        onClick={onRemove}
        className="text-destructive-foreground absolute -top-1 -right-1 rounded-full bg-destructive p-0.5"
      >
        <X className="h-3 w-3" />
      </button>
    </div>
  )
}
