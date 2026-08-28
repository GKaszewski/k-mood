import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { toast } from "@/components/ui/toast"
import { useNowPlaying } from "@/hooks/use-providers"
import type { SongNote } from "@/lib/entry-draft"

interface SongFieldProps {
  value: SongNote
  onChange: (song: SongNote) => void
}

export function SongField({ value, onChange }: SongFieldProps) {
  const nowPlaying = useNowPlaying()

  const fillFromNowPlaying = () => {
    nowPlaying.mutate(undefined, {
      onSuccess: (playing) => {
        if (playing?.kind !== "song") {
          toast.add({ title: "Nothing is playing right now", type: "info" })
          return
        }
        onChange({ title: playing.title, artist: playing.artist })
      },
      onError: () =>
        toast.add({
          title: "Could not reach your music server",
          type: "error",
        }),
    })
  }

  return (
    <div className="flex flex-col gap-2">
      <Label htmlFor="song-title" className="sr-only">
        Song title
      </Label>
      <Input
        id="song-title"
        placeholder="Title"
        value={value.title}
        onChange={(event) => onChange({ ...value, title: event.target.value })}
      />
      <Label htmlFor="song-artist" className="sr-only">
        Artist
      </Label>
      <Input
        id="song-artist"
        placeholder="Artist"
        value={value.artist}
        onChange={(event) => onChange({ ...value, artist: event.target.value })}
      />
      <Button
        type="button"
        variant="outline"
        size="sm"
        className="self-start"
        onClick={fillFromNowPlaying}
        disabled={nowPlaying.isPending}
      >
        {nowPlaying.isPending ? "Checking…" : "What's playing?"}
      </Button>
    </div>
  )
}
