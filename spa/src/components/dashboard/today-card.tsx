import { Link } from "@tanstack/react-router"
import { isToday } from "date-fns"
import { buttonLook } from "@/lib/button-style"
import { Card, CardContent } from "@/components/ui/card"
import { MoodBadge } from "@/components/mood/mood-badge"
import type { EntryResponse } from "@/api/schema"

/**
 * How today has gone, or an invitation to say. The latest entry is only
 * "today's mood" if it was logged today — showing a mood from last week under
 * that heading was telling the reader something untrue.
 */
export function TodayCard({ latest }: { latest?: EntryResponse }) {
  const loggedToday = latest && isToday(new Date(latest.loggedAt))

  if (!loggedToday) {
    return (
      <Card>
        <CardContent className="flex flex-col items-center gap-3 py-6">
          <p className="text-sm text-muted-foreground">
            Nothing logged today yet
          </p>
          <Link to="/add" className={buttonLook()}>
            How are you feeling?
          </Link>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardContent className="flex flex-col items-center gap-2 py-6">
        <p className="text-sm text-muted-foreground">Today</p>
        <MoodBadge value={latest.mood} showLabel className="text-2xl" />
      </CardContent>
    </Card>
  )
}
