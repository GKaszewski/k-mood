import { createFileRoute, Link } from "@tanstack/react-router"
import { format } from "date-fns"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { EntryList } from "@/components/entry/entry-list"
import { PendingEntries } from "@/components/entry/pending-entries"
import { MoodSummary } from "@/components/stats/mood-summary"
import { TodayCard } from "@/components/dashboard/today-card"
import { MetricsCard } from "@/components/dashboard/metrics-card"
import { SetupCard } from "@/components/dashboard/setup-card"
import { useRecentEntries } from "@/hooks/use-entries"
import { useMoodStats } from "@/hooks/use-stats"
import { useAuth } from "@/hooks/use-auth"

export const Route = createFileRoute("/_auth/")({ component: DashboardPage })

const RECENT_SHOWN = 10

function DashboardPage() {
  const { user } = useAuth()
  const { data: recent, isLoading: entriesLoading } =
    useRecentEntries(RECENT_SHOWN)
  const { data: stats, isLoading: statsLoading } = useMoodStats("all")

  const entries = recent?.items ?? []

  return (
    <div className="flex flex-col gap-6 py-6">
      <header>
        <p className="text-sm text-muted-foreground">
          {format(new Date(), "EEEE, MMMM d")}
        </p>
        <h1 className="text-2xl font-bold">
          Hey{user ? `, ${user.displayName ?? user.username}` : ""}
        </h1>
      </header>

      <PendingEntries />

      <TodayCard latest={entries[0]} />

      <MetricsCard />

      {stats && <SetupCard totalEntries={stats.totalEntries} />}

      {statsLoading ? (
        <Skeleton className="h-24" />
      ) : stats ? (
        <MoodSummary stats={stats} />
      ) : null}

      <section>
        <div className="mb-3 flex items-center justify-between">
          <h2 className="text-lg font-semibold">Recent entries</h2>
          <Link to="/diary" className="text-sm text-primary hover:underline">
            View all
          </Link>
        </div>
        {entriesLoading ? (
          <div className="flex flex-col gap-2">
            <Skeleton className="h-20" />
            <Skeleton className="h-20" />
          </div>
        ) : entries.length ? (
          <EntryList entries={entries} />
        ) : (
          <Card>
            <CardContent className="py-8 text-center text-sm text-muted-foreground">
              No entries yet. Tap ＋ to log your first mood.
            </CardContent>
          </Card>
        )}
      </section>
    </div>
  )
}
