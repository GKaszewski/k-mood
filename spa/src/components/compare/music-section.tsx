import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import { formatShare } from "@/lib/compare-format"
import type {
  MusicComparisonResponse,
  MusicResponse,
  RankedResponse,
  RankedSongResponse,
} from "@/api/schema"

/** Top lists side by side. Music is described, never tested. */
export function MusicSection({ music }: { music: MusicComparisonResponse }) {
  const empty = !music.a.entriesWithSong && !music.b.entriesWithSong
  const newInB = new Set(music.onlyInB)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Music</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {empty ? (
          <p className="text-xs text-muted-foreground">
            No songs on entries in either period.
          </p>
        ) : (
          <>
            <p className="text-xs text-muted-foreground">
              Shares are of entries that carry a song.
            </p>
            <div className="grid grid-cols-2 gap-4">
              <Side side="A" color={PERIOD_A_COLOR} music={music.a} />
              <Side
                side="B"
                color={PERIOD_B_COLOR}
                music={music.b}
                isNew={(artist) => newInB.has(artist)}
              />
            </div>
            {music.onlyInA.length > 0 && (
              <div className="flex flex-col gap-1">
                <h4 className="text-xs font-medium">Gone in B</h4>
                <ul className="flex flex-wrap gap-1">
                  {music.onlyInA.map((artist) => (
                    <li key={artist}>
                      <Badge variant="outline">{artist}</Badge>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </>
        )}
      </CardContent>
    </Card>
  )
}

function Side({
  side,
  color,
  music,
  isNew,
}: {
  side: "A" | "B"
  color: string
  music: MusicResponse
  isNew?: (artist: string) => boolean
}) {
  return (
    <div className="flex min-w-0 flex-col gap-3">
      <h4 className="text-xs font-medium">
        <span aria-hidden="true" style={{ color }}>
          ●{" "}
        </span>
        {side}
        <span className="ml-1 font-normal text-muted-foreground">
          ({music.entriesWithSong} with a song)
        </span>
      </h4>
      <RankedList
        title="Artists"
        items={music.artists}
        nameOf={(artist: RankedResponse) => artist.name}
        isNew={isNew}
      />
      <RankedList
        title="Songs"
        items={music.songs}
        nameOf={(song: RankedSongResponse) => `${song.title} — ${song.artist}`}
      />
    </div>
  )
}

function RankedList<TItem extends { share: number }>({
  title,
  items,
  nameOf,
  isNew,
}: {
  title: string
  items: TItem[]
  nameOf: (item: TItem) => string
  isNew?: (name: string) => boolean
}) {
  if (!items.length) return null

  return (
    <div className="flex flex-col gap-1">
      <span className="text-[10px] text-muted-foreground uppercase">
        {title}
      </span>
      <ol className="flex flex-col gap-0.5">
        {items.map((item) => {
          const name = nameOf(item)
          return (
            <li
              key={name}
              className="flex items-center justify-between gap-2 text-xs"
            >
              <span className="flex min-w-0 items-center gap-1">
                <span className="truncate">{name}</span>
                {isNew?.(name) && <Badge variant="secondary">new in B</Badge>}
              </span>
              <span className="shrink-0 text-muted-foreground tabular-nums">
                {formatShare(item.share)}
              </span>
            </li>
          )
        })}
      </ol>
    </div>
  )
}
