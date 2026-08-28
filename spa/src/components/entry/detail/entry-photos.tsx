import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { PhotoLightbox } from "@/components/media/photo-lightbox"
import { media } from "@/api/client"

export function EntryPhotos({ photoIds }: { photoIds: string[] }) {
  const [openAt, setOpenAt] = useState<number | null>(null)
  const urls = photoIds.map(media.photoUrl)

  if (!photoIds.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Photos</CardTitle>
      </CardHeader>
      <CardContent className="grid grid-cols-2 gap-2">
        {urls.map((url, index) => (
          <button
            key={photoIds[index]}
            type="button"
            aria-label={`Open photo ${index + 1} of ${urls.length}`}
            onClick={() => setOpenAt(index)}
            className="aspect-square overflow-hidden rounded-lg focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
          >
            <img
              src={url}
              alt={`Photo ${index + 1}`}
              loading="lazy"
              className="h-full w-full object-cover"
            />
          </button>
        ))}
      </CardContent>

      <PhotoLightbox
        urls={urls}
        openAt={openAt}
        onClose={() => setOpenAt(null)}
      />
    </Card>
  )
}
