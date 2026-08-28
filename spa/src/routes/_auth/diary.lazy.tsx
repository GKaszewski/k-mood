import { useState } from "react"
import { createLazyFileRoute } from "@tanstack/react-router"
import { SlidersHorizontal } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { Spinner } from "@/components/ui/spinner"
import { EntryList } from "@/components/entry/entry-list"
import { PendingEntries } from "@/components/entry/pending-entries"
import { EntryFiltersBar, isNarrowed } from "@/components/entry/entry-filters"
import { useEntryFeed } from "@/hooks/use-entries"
import type { EntryFilters } from "@/hooks/use-entries"
import { useInfiniteScroll } from "@/hooks/use-infinite-scroll"

export const Route = createLazyFileRoute("/_auth/diary")({
  component: DiaryPage,
})

function DiaryPage() {
  const [filters, setFilters] = useState<EntryFilters>({})
  const [filtersOpen, setFiltersOpen] = useState(false)
  const feed = useEntryFeed(filters)

  const entries = feed.data?.pages.flatMap((page) => page.items) ?? []
  const total = feed.data?.pages[0]?.total ?? 0

  const sentinelRef = useInfiniteScroll({
    hasMore: feed.hasNextPage,
    isLoading: feed.isFetchingNextPage,
    loadMore: feed.fetchNextPage,
  })

  return (
    <div className="flex flex-col gap-4 py-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold">Diary</h1>
          {!feed.isLoading && (
            <p className="text-xs text-muted-foreground">
              {countOf(total, isNarrowed(filters))}
            </p>
          )}
        </div>
        <Button
          variant={isNarrowed(filters) ? "default" : "outline"}
          size="sm"
          aria-expanded={filtersOpen}
          onClick={() => setFiltersOpen((open) => !open)}
        >
          <SlidersHorizontal className="mr-1 h-4 w-4" aria-hidden="true" />
          Filter
        </Button>
      </header>

      {filtersOpen && (
        <Card>
          <CardContent className="py-4">
            <EntryFiltersBar filters={filters} onChange={setFilters} />
          </CardContent>
        </Card>
      )}

      {!isNarrowed(filters) && <PendingEntries />}

      {feed.isLoading ? (
        <div className="flex flex-col gap-2">
          <Skeleton className="h-20" />
          <Skeleton className="h-20" />
          <Skeleton className="h-20" />
        </div>
      ) : entries.length ? (
        <EntryList entries={entries} />
      ) : (
        <Card>
          <CardContent className="py-8 text-center text-sm text-muted-foreground">
            {isNarrowed(filters)
              ? "Nothing matches these filters."
              : "No entries yet."}
          </CardContent>
        </Card>
      )}

      <div ref={sentinelRef} className="flex justify-center py-2">
        {feed.isFetchingNextPage && <Spinner />}
      </div>
    </div>
  )
}

function countOf(total: number, narrowed: boolean): string {
  if (narrowed) return `${total} ${total === 1 ? "match" : "matches"}`

  return `${total} ${total === 1 ? "entry" : "entries"}`
}
