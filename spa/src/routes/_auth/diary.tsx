import { useState } from "react"
import { createFileRoute } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { EntryList } from "@/components/entry/entry-list"
import { useEntries } from "@/hooks/use-entries"

const PAGE_SIZE = 30

export const Route = createFileRoute("/_auth/diary")({
  component: DiaryPage,
})

function DiaryPage() {
  const [page, setPage] = useState(0)
  const { data: entries, isLoading } = useEntries({
    limit: PAGE_SIZE,
    offset: page * PAGE_SIZE,
  })

  const hasMore = (entries?.length ?? 0) === PAGE_SIZE

  return (
    <div className="flex flex-col gap-4 py-6">
      <header>
        <h1 className="text-xl font-bold">Diary</h1>
      </header>

      {isLoading ? (
        <div className="flex flex-col gap-2">
          <Skeleton className="h-20" />
          <Skeleton className="h-20" />
          <Skeleton className="h-20" />
        </div>
      ) : entries?.length ? (
        <EntryList entries={entries} />
      ) : (
        <Card>
          <CardContent className="py-8 text-center text-sm text-muted-foreground">
            {page > 0 ? "No more entries" : "No entries yet"}
          </CardContent>
        </Card>
      )}

      <div className="flex items-center justify-between">
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage((p) => Math.max(0, p - 1))}
          disabled={page === 0}
        >
          Newer
        </Button>
        <span className="text-xs text-muted-foreground">Page {page + 1}</span>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage((p) => p + 1)}
          disabled={!hasMore}
        >
          Older
        </Button>
      </div>
    </div>
  )
}
