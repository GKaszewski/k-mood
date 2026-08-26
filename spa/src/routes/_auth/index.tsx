import { createFileRoute, Link } from "@tanstack/react-router"
import { format } from "date-fns"
import { Card, CardContent } from "@/components/ui/card"
import { Footprints } from "lucide-react"
import { MoodBadge } from "@/components/mood/mood-badge"
import { EntryList } from "@/components/entry/entry-list"
import { useEntries, useMoodStats } from "@/hooks/use-entries"
import { byKind, useDailyMetrics } from "@/hooks/use-metrics"
import { useAuth } from "@/hooks/use-auth"
import { Skeleton } from "@/components/ui/skeleton"

export const Route = createFileRoute("/_auth/")({ component: DashboardPage })

function DashboardPage() {
  const { user } = useAuth()
  const { data: entries, isLoading: entriesLoading } = useEntries({ limit: 10 })
  const { data: stats, isLoading: statsLoading } = useMoodStats()

  const today = format(new Date(), "EEEE, MMMM d")
  const todaysDate = format(new Date(), "yyyy-MM-dd")
  const { data: todaysMetrics } = useDailyMetrics(todaysDate, todaysDate)
  const steps = byKind(todaysMetrics).get("steps")
  const latestMood = entries?.[0]?.mood

  return (
    <div className="flex flex-col gap-6 py-6">
      <header>
        <p className="text-sm text-muted-foreground">{today}</p>
        <h1 className="text-2xl font-bold">
          Hey{user ? `, ${user.displayName ?? user.username}` : ""}
        </h1>
      </header>

      {latestMood && (
        <Card>
          <CardContent className="flex flex-col items-center gap-2 py-6">
            <p className="text-sm text-muted-foreground">Current mood</p>
            <MoodBadge value={latestMood} showLabel className="text-2xl" />
          </CardContent>
        </Card>
      )}

      <Link to="/metrics">
        <Card>
          <CardContent className="flex items-center justify-between py-4">
            <div className="flex items-center gap-2">
              <Footprints className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm">Steps today</span>
            </div>
            <span className="text-sm font-semibold">
              {steps ? steps.value.toLocaleString() : "Add"}
            </span>
          </CardContent>
        </Card>
      </Link>

      {statsLoading ? (
        <Skeleton className="h-24" />
      ) : stats ? (
        <div className="grid grid-cols-3 gap-3">
          <StatCard
            label="Average"
            value={stats.average ? stats.average.toFixed(1) : "—"}
          />
          <StatCard label="Streak" value={`${stats.currentStreak}d`} />
          <StatCard label="Entries" value={String(stats.totalEntries)} />
        </div>
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
        ) : entries?.length ? (
          <EntryList entries={entries} limit={10} />
        ) : (
          <Card>
            <CardContent className="py-8 text-center text-sm text-muted-foreground">
              No entries yet. Tap + to log your first mood!
            </CardContent>
          </Card>
        )}
      </section>
    </div>
  )
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <Card>
      <CardContent className="flex flex-col items-center py-4">
        <span className="text-2xl font-bold">{value}</span>
        <span className="text-xs text-muted-foreground">{label}</span>
      </CardContent>
    </Card>
  )
}
