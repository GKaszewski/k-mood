import { format } from "date-fns"
import { Link } from "@tanstack/react-router"
import { Card, CardContent } from "@/components/ui/card"
import { MoodBadge } from "@/components/mood/mood-badge"
import type { EntryResponse } from "@/api/schema"

interface EntryCardProps {
  entry: EntryResponse
}

export function EntryCard({ entry }: EntryCardProps) {
  const date = new Date(entry.loggedAt)

  return (
    <Link to="/entry/$id" params={{ id: entry.id }}>
      <Card className="cursor-pointer transition-colors hover:bg-muted/50">
        <CardContent className="flex items-start gap-3 p-4">
          <MoodBadge value={entry.mood} />
          <div className="min-w-0 flex-1">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">{entry.moodLabel}</span>
              <span className="text-xs text-muted-foreground">
                {format(date, "h:mm a")}
              </span>
            </div>
            {entry.content && (
              <p className="mt-1 line-clamp-2 text-sm text-muted-foreground">
                {entry.content}
              </p>
            )}
          </div>
        </CardContent>
      </Card>
    </Link>
  )
}
