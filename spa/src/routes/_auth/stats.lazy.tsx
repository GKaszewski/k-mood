import { useState } from "react"
import { createLazyFileRoute } from "@tanstack/react-router"
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Skeleton } from "@/components/ui/skeleton"
import { MoodTrendChart } from "@/components/stats/mood-trend-chart"
import { MoodDistribution } from "@/components/stats/mood-distribution"
import { MoodCorrelations } from "@/components/stats/mood-correlations"
import { MoodSummary } from "@/components/stats/mood-summary"
import { HeatmapSection } from "@/components/stats/heatmap-section"
import { MetricsLink } from "@/components/stats/metrics-link"
import { CompareLink } from "@/components/stats/compare-link"
import { useMoodStats } from "@/hooks/use-stats"
import { useTrend } from "@/hooks/use-trend"
import { PERIODS, PERIOD_LABELS } from "@/lib/period"
import type { Period } from "@/lib/period"

export const Route = createLazyFileRoute("/_auth/stats")({
  component: StatsPage,
})

function StatsPage() {
  const [period, setPeriod] = useState<Period>("month")
  const { data: stats, isLoading } = useMoodStats(period)
  const { data: trend } = useTrend(period)

  return (
    <div className="flex flex-col gap-4 py-6">
      <h1 className="text-xl font-bold">Statistics</h1>

      <Tabs
        value={period}
        onValueChange={(value) => setPeriod(value as Period)}
      >
        <TabsList className="grid w-full grid-cols-4">
          {PERIODS.map((option) => (
            <TabsTrigger key={option} value={option}>
              {PERIOD_LABELS[option]}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>

      {isLoading ? (
        <div className="flex flex-col gap-4">
          <Skeleton className="h-24" />
          <Skeleton className="h-48" />
        </div>
      ) : stats ? (
        <>
          <MoodSummary stats={stats} />

          {trend ? <MoodTrendChart trend={trend} /> : null}

          {stats.frequency.length ? (
            <MoodDistribution frequency={stats.frequency} />
          ) : null}

          <MoodCorrelations />
        </>
      ) : null}

      <HeatmapSection />

      <MetricsLink />

      <CompareLink />
    </div>
  )
}
