import { useState } from "react"
import { createLazyFileRoute, useRouter } from "@tanstack/react-router"
import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { DayStepper } from "@/components/metric/day-stepper"
import { MetricsEditor } from "@/components/metric/metrics-editor"
import { useDailyMetrics } from "@/hooks/use-metrics"
import { today } from "@/lib/day"

export const Route = createLazyFileRoute("/_auth/metrics")({
  component: MetricsPage,
})

function MetricsPage() {
  const router = useRouter()
  const [date, setDate] = useState(today)

  return (
    <div className="flex flex-col gap-6 py-6">
      <header className="flex items-center gap-2">
        <Button
          variant="ghost"
          size="icon"
          aria-label="Go back"
          onClick={() => router.history.back()}
        >
          <ArrowLeft className="h-5 w-5" aria-hidden="true" />
        </Button>
        <h1 className="text-2xl font-bold">Daily metrics</h1>
      </header>

      <DayStepper value={date} onChange={setDate} />

      <DayMetrics key={date} date={date} />
    </div>
  )
}

function DayMetrics({ date }: { date: string }) {
  const { data: stored, isLoading } = useDailyMetrics(date, date)

  if (isLoading || !stored) {
    return (
      <div className="flex flex-col gap-4">
        <Skeleton className="h-14" />
        <Skeleton className="h-14" />
        <Skeleton className="h-14" />
      </div>
    )
  }

  return <MetricsEditor date={date} stored={stored} />
}
