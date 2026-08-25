import { useRef } from "react"
import { ImagePlus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { MediaThumbnail } from "./media-thumbnail"
import { media } from "@/api/client"
import type { MediaItem } from "@/lib/media"
import { createPendingItem } from "@/lib/media"

interface PhotoPickerProps {
  items: MediaItem[]
  onChange: (items: MediaItem[]) => void
}

export function PhotoPicker({ items, onChange }: PhotoPickerProps) {
  const inputRef = useRef<HTMLInputElement>(null)

  const handleFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return
    onChange([...items, createPendingItem(file)])
    e.target.value = ""
  }

  const remove = (index: number) => {
    onChange(items.filter((_, i) => i !== index))
  }

  return (
    <div className="flex flex-col gap-3">
      {items.length > 0 && (
        <div className="flex flex-wrap gap-2">
          {items.map((item, i) => (
            <MediaThumbnail
              key={i}
              src={
                item.type === "existing"
                  ? media.photoUrl(item.id)
                  : item.preview
              }
              onRemove={() => remove(i)}
            />
          ))}
        </div>
      )}

      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        onChange={handleFile}
        className="hidden"
      />

      <Button
        type="button"
        variant="outline"
        size="sm"
        onClick={() => inputRef.current?.click()}
      >
        <ImagePlus className="mr-2 h-4 w-4" />
        Add photo
      </Button>
    </div>
  )
}
