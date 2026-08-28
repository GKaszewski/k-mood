import { useRef } from "react"
import { ImagePlus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { MediaThumbnail } from "./media-thumbnail"
import { media } from "@/api/client"
import { usePreviewUrls } from "@/hooks/use-preview-urls"
import { keyOf, newPendingItem } from "@/lib/media"
import type { MediaItem } from "@/lib/media"

interface PhotoPickerProps {
  items: MediaItem[]
  onChange: (items: MediaItem[]) => void
}

export function PhotoPicker({ items, onChange }: PhotoPickerProps) {
  const inputRef = useRef<HTMLInputElement>(null)
  const previews = usePreviewUrls(items)

  const handleFile = (event: React.ChangeEvent<HTMLInputElement>) => {
    const chosen = Array.from(event.target.files ?? [])
    if (chosen.length) onChange([...items, ...chosen.map(newPendingItem)])
    event.target.value = ""
  }

  const removeAt = (index: number) => {
    onChange(items.filter((_, position) => position !== index))
  }

  return (
    <div className="flex flex-col gap-3">
      {items.length > 0 && (
        <ul className="flex list-none flex-wrap gap-2 p-0">
          {items.map((item, index) => (
            <li key={keyOf(item)}>
              <MediaThumbnail
                src={
                  item.kind === "existing"
                    ? media.photoUrl(item.id)
                    : (previews.get(item.localId) ?? "")
                }
                alt={`Photo ${index + 1}`}
                onRemove={() => removeAt(index)}
              />
            </li>
          ))}
        </ul>
      )}

      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        multiple
        onChange={handleFile}
        className="hidden"
      />

      <Button
        type="button"
        variant="outline"
        size="sm"
        onClick={() => inputRef.current?.click()}
      >
        <ImagePlus className="mr-2 h-4 w-4" aria-hidden="true" />
        Add photo
      </Button>
    </div>
  )
}
