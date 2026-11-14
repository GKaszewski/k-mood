import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import { formatDay, formatValue } from "@/lib/compare-format"
import { BEST_MOOD, WORST_MOOD, getMoodEmoji } from "@/lib/mood"
import type {
  ComparedTrendPointResponse,
  ComparedTrendResponse,
} from "@/api/schema"

const AXIS_PADDING = 0.5
const CHART_HEIGHT = 220
const MAX_POINTS_WITH_DOTS = 60

const UNIT_OF_GRANULARITY: Record<string, string> = {
  day: "Day",
  week: "Week",
  month: "30 days",
}

/** Bucket n of A beside bucket n of B. Both count from their own first day. */
interface Aligned {
  offset: number
  a: ComparedTrendPointResponse | undefined
  b: ComparedTrendPointResponse | undefined
  moodA: number | null
  moodB: number | null
}

function align(trend: ComparedTrendResponse): Aligned[] {
  const length = Math.max(trend.a.length, trend.b.length)
  return Array.from({ length }, (_, offset) => {
    const a = trend.a.find((point) => point.offset === offset)
    const b = trend.b.find((point) => point.offset === offset)
    return {
      offset,
      a,
      b,
      moodA: a?.meanMood ?? null,
      moodB: b?.meanMood ?? null,
    }
  })
}

function bucketLabel(granularity: string, offset: number): string {
  return `${UNIT_OF_GRANULARITY[granularity] ?? "Bucket"} ${offset + 1}`
}

export function TrendSection({ trend }: { trend: ComparedTrendResponse }) {
  const data = align(trend)
  if (data.length === 0) return null

  const showDots = data.length <= MAX_POINTS_WITH_DOTS
  const dot = (color: string) =>
    showDots
      ? { r: 2.5, fill: "var(--popover)", stroke: color, strokeWidth: 1.5 }
      : false

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood trend</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        <p className="text-xs text-muted-foreground">
          Both periods start from their own first day. Gaps are periods with
          nothing logged.
        </p>
        <div
          role="img"
          aria-label={`Average mood per ${trend.granularity}, period A in green and period B in blue, each aligned on its own first day.`}
        >
          <ResponsiveContainer width="100%" height={CHART_HEIGHT}>
            <LineChart
              data={data}
              margin={{ top: 5, right: 5, bottom: 0, left: -10 }}
            >
              <CartesianGrid
                strokeDasharray="3 3"
                stroke="var(--border)"
                vertical={false}
              />
              <XAxis
                dataKey="offset"
                tickFormatter={(offset: number) => String(offset + 1)}
                tick={{ fontSize: 11, fill: "var(--foreground)" }}
                tickLine={false}
                axisLine={false}
                interval="preserveStartEnd"
                minTickGap={30}
              />
              <YAxis
                domain={[
                  WORST_MOOD.value - AXIS_PADDING,
                  BEST_MOOD.value + AXIS_PADDING,
                ]}
                ticks={[1, 2, 3, 4, 5]}
                tick={{ fontSize: 14 }}
                tickFormatter={getMoodEmoji}
                tickLine={false}
                axisLine={false}
                width={32}
              />
              <Tooltip
                content={<CompareTooltip granularity={trend.granularity} />}
              />
              <Line
                type="monotone"
                dataKey="moodA"
                name="A"
                stroke={PERIOD_A_COLOR}
                strokeWidth={2}
                connectNulls={false}
                dot={dot(PERIOD_A_COLOR)}
                activeDot={{ r: 4, fill: PERIOD_A_COLOR }}
              />
              <Line
                type="monotone"
                dataKey="moodB"
                name="B"
                stroke={PERIOD_B_COLOR}
                strokeWidth={2}
                connectNulls={false}
                dot={dot(PERIOD_B_COLOR)}
                activeDot={{ r: 4, fill: PERIOD_B_COLOR }}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </CardContent>
    </Card>
  )
}

function CompareTooltip(props: { granularity: string }) {
  const { active, payload, granularity } = props as {
    active?: boolean
    payload?: ReadonlyArray<{ payload: Aligned }>
    granularity: string
  }

  if (!active || !payload?.length) return null

  const bucket = payload[0].payload

  return (
    <div className="rounded-lg border bg-popover px-3 py-2 text-sm text-popover-foreground shadow-md">
      <p className="font-medium">{bucketLabel(granularity, bucket.offset)}</p>
      <TooltipLine side="A" color={PERIOD_A_COLOR} point={bucket.a} />
      <TooltipLine side="B" color={PERIOD_B_COLOR} point={bucket.b} />
    </div>
  )
}

function TooltipLine({
  side,
  color,
  point,
}: {
  side: string
  color: string
  point: ComparedTrendPointResponse | undefined
}) {
  if (!point) return null

  const when =
    point.daysCovered > 1
      ? `from ${formatDay(point.start)}`
      : formatDay(point.start)

  return (
    <p className="flex items-center gap-1.5">
      <span aria-hidden="true" style={{ color }}>
        ●
      </span>
      <span>
        {side} · {when}:{" "}
        {point.meanMood === null
          ? "nothing logged"
          : `${formatValue("mood", point.meanMood)} avg`}
        <span className="ml-1 text-muted-foreground">
          ({point.entries} {point.entries === 1 ? "entry" : "entries"})
        </span>
      </span>
    </p>
  )
}
