import { useMemo } from "react"
import { createLazyFileRoute, useRouter } from "@tanstack/react-router"
import { ArrowLeft, Info } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { PeriodPicker } from "@/components/compare/period-picker"
import { PresetChips } from "@/components/compare/preset-chips"
import { OverviewSection } from "@/components/compare/overview-section"
import { TrendSection } from "@/components/compare/trend-section"
import { HeatmapSection } from "@/components/compare/heatmap-section"
import { MoodSection } from "@/components/compare/mood-section"
import { ActivitiesSection } from "@/components/compare/activities-section"
import { MetricsSection } from "@/components/compare/metrics-section"
import { WeatherSection } from "@/components/compare/weather-section"
import { MusicSection } from "@/components/compare/music-section"
import { CycleSection } from "@/components/compare/cycle-section"
import { useComparison } from "@/hooks/use-comparison"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import { sameMonthLastYear } from "@/lib/compare-presets"
import type { DaySpan, SpanPair } from "@/lib/compare-presets"
import { messageFor } from "@/lib/api-message"

export const Route = createLazyFileRoute("/_auth/compare")({
  component: ComparePage,
})

function ComparePage() {
  const router = useRouter()
  const navigate = Route.useNavigate()
  const search = Route.useSearch()

  const [a, b] = useMemo<SpanPair>(() => {
    const { aFrom, aTo, bFrom, bTo } = search
    if (aFrom && aTo && bFrom && bTo) {
      return [
        { from: aFrom, to: aTo },
        { from: bFrom, to: bTo },
      ]
    }
    return sameMonthLastYear(new Date())
  }, [search])

  const { data, isLoading, isPlaceholderData, error } = useComparison(a, b)
  const current = isPlaceholderData ? undefined : data

  const show = ([nextA, nextB]: SpanPair) =>
    navigate({
      search: {
        aFrom: nextA.from,
        aTo: nextA.to,
        bFrom: nextB.from,
        bTo: nextB.to,
      },
    })

  return (
    <div className="flex flex-col gap-4 py-6">
      <header className="flex items-center gap-2">
        <Button
          variant="ghost"
          size="icon"
          aria-label="Go back"
          onClick={() => router.history.back()}
        >
          <ArrowLeft className="h-5 w-5" aria-hidden="true" />
        </Button>
        <h1 className="text-xl font-bold">Compare periods</h1>
      </header>

      <div className="flex flex-col gap-2">
        <div className="flex flex-wrap items-center gap-2">
          <PeriodPicker
            side="A"
            color={PERIOD_A_COLOR}
            span={a}
            effective={current?.a}
            onChange={(next: DaySpan) => show([next, b])}
          />
          <span className="text-xs text-muted-foreground">vs</span>
          <PeriodPicker
            side="B"
            color={PERIOD_B_COLOR}
            span={b}
            effective={current?.b}
            onChange={(next: DaySpan) => show([a, next])}
          />
        </div>
        <PresetChips a={a} b={b} effectiveB={current?.b} onPick={show} />
      </div>

      {isLoading ? (
        <div className="flex flex-col gap-4">
          <Skeleton className="h-40" />
          <Skeleton className="h-56" />
          <Skeleton className="h-48" />
        </div>
      ) : error ? (
        <Alert variant="destructive">
          <AlertTitle>Could not compare these periods</AlertTitle>
          <AlertDescription>{messageFor(error)}</AlertDescription>
        </Alert>
      ) : data ? (
        <div
          className="flex flex-col gap-4 transition-opacity aria-busy:opacity-60"
          aria-busy={isPlaceholderData}
        >
          {data.spansOverlap && (
            <Alert>
              <Info aria-hidden="true" />
              <AlertTitle>The periods overlap</AlertTitle>
              <AlertDescription>
                Some days fall in both periods, so the two samples are not
                independent and no change is marked as likely real or noise. The
                figures are still shown.
              </AlertDescription>
            </Alert>
          )}
          <OverviewSection rows={data.overview} />
          <TrendSection trend={data.trend} />
          <HeatmapSection a={data.a} b={data.b} />
          <MoodSection section={data.mood} />
          <ActivitiesSection rows={data.activities} />
          <MetricsSection rows={data.metrics} />
          <WeatherSection rows={data.weather} />
          <MusicSection music={data.music} />
          {data.cycle && <CycleSection rows={data.cycle} />}
        </div>
      ) : null}
    </div>
  )
}
