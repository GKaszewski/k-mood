import { useState } from "react"
import { createLazyFileRoute } from "@tanstack/react-router"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { getMoodEmoji } from "@/lib/mood"
import { Button } from "@/components/ui/button"
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Skeleton } from "@/components/ui/skeleton"
import { MoodTrendChart } from "@/components/stats/mood-trend-chart"
import { MoodDistribution } from "@/components/stats/mood-distribution"
import { MoodHeatmap } from "@/components/stats/mood-heatmap"
import { ActivityCorrelations } from "@/components/stats/activity-correlations"
import { StatCard } from "@/components/stats/stat-card"
import { useMoodStats, usePeriodEntries } from "@/hooks/use-stats"
import { useYearCalendar } from "@/hooks/use-year-calendar"

export const Route = createLazyFileRoute("/_auth/stats")({
  component: StatsPage,
})

type Period = "week" | "month" | "year" | "all"

function StatsPage() {
  const [period, setPeriod] = useState<Period>("month")
  const [heatmapYear, setHeatmapYear] = useState(new Date().getFullYear())
  const { data: stats, isLoading: statsLoading } = useMoodStats(period)
  const { data: trendEntries } = usePeriodEntries(period)
  const { data: yearDays, isLoading: heatmapLoading } =
    useYearCalendar(heatmapYear)

  return (
    <div className="flex flex-col gap-4 py-6">
      <h1 className="text-xl font-bold">Statistics</h1>

      <Tabs value={period} onValueChange={(v) => setPeriod(v as Period)}>
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="week">Week</TabsTrigger>
          <TabsTrigger value="month">Month</TabsTrigger>
          <TabsTrigger value="year">Year</TabsTrigger>
          <TabsTrigger value="all">All</TabsTrigger>
        </TabsList>
      </Tabs>

      {statsLoading ? (
        <div className="flex flex-col gap-4">
          <Skeleton className="h-24" />
          <Skeleton className="h-48" />
        </div>
      ) : stats ? (
        <>
          <div className="grid grid-cols-3 gap-3">
            <StatCard
              label="Average"
              value={
                stats.average
                  ? getMoodEmoji(Math.round(Number(stats.average)))
                  : "—"
              }
            />
            <StatCard label="Streak" value={`${stats.currentStreak}d`} />
            <StatCard label="Entries" value={stats.totalEntries ?? 0} />
          </div>

          {trendEntries?.length ? (
            <MoodTrendChart entries={trendEntries} period={period} />
          ) : null}

          {stats.frequency?.length ? (
            <MoodDistribution frequency={stats.frequency} />
          ) : null}

          <ActivityCorrelations />
        </>
      ) : null}

      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setHeatmapYear((y) => y - 1)}
        >
          <ChevronLeft className="h-5 w-5" />
        </Button>
        <span className="text-sm font-medium">{heatmapYear}</span>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setHeatmapYear((y) => y + 1)}
          disabled={heatmapYear >= new Date().getFullYear()}
        >
          <ChevronRight className="h-5 w-5" />
        </Button>
      </div>

      {heatmapLoading ? (
        <Skeleton className="h-32" />
      ) : yearDays ? (
        <MoodHeatmap year={heatmapYear} days={yearDays} />
      ) : null}
    </div>
  )
}
