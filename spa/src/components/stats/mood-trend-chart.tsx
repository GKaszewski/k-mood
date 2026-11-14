import { useMemo } from "react"
import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { BEST_MOOD, MIDDLE_MOOD, WORST_MOOD, getMoodEmoji } from "@/lib/mood"
import { barsOf } from "@/lib/trend-labels"
import type { TrendBar } from "@/lib/trend-labels"
import type { MoodTrendResponse } from "@/api/schema"

const GRADIENT_ID = "moodTrendGradient"
const MAX_POINTS_WITH_DOTS = 60
const AXIS_PADDING = 0.5

interface MoodTrendChartProps {
  trend: MoodTrendResponse
  title?: string
}

export function MoodTrendChart({
  trend,
  title = "Mood over time",
}: MoodTrendChartProps) {
  const bars = useMemo(() => barsOf(trend.granularity, trend.points), [trend])

  if (!bars.some((bar) => bar.average !== null)) return null

  const showDots = bars.length <= MAX_POINTS_WITH_DOTS

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={220}>
          <AreaChart
            data={bars}
            margin={{ top: 5, right: 5, bottom: 0, left: -10 }}
          >
            <defs>
              <linearGradient id={GRADIENT_ID} x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="0%"
                  stopColor={BEST_MOOD.swatch}
                  stopOpacity={0.4}
                />
                <stop
                  offset="50%"
                  stopColor={MIDDLE_MOOD.swatch}
                  stopOpacity={0.2}
                />
                <stop
                  offset="100%"
                  stopColor={WORST_MOOD.swatch}
                  stopOpacity={0.05}
                />
              </linearGradient>
            </defs>
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="var(--border)"
              vertical={false}
            />
            <XAxis
              dataKey="label"
              tick={{ fontSize: 11, fill: "var(--foreground)" }}
              tickLine={false}
              axisLine={false}
              interval="preserveStartEnd"
              minTickGap={50}
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
            <Tooltip content={<TrendTooltip />} />
            <Area
              type="monotone"
              dataKey="average"
              stroke={BEST_MOOD.swatch}
              fill={`url(#${GRADIENT_ID})`}
              strokeWidth={2}
              dot={
                showDots
                  ? {
                      r: 2.5,
                      fill: "var(--popover)",
                      stroke: BEST_MOOD.swatch,
                      strokeWidth: 1.5,
                    }
                  : false
              }
              activeDot={{ r: 4, fill: BEST_MOOD.swatch }}
              connectNulls={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function TrendTooltip(props: unknown) {
  const { active, payload } = props as {
    active?: boolean
    payload?: ReadonlyArray<{ payload: TrendBar }>
  }

  if (!active || !payload?.length) return null

  const bar = payload[0].payload
  if (bar.average === null) return null

  return (
    <div className="rounded-lg border bg-popover px-3 py-2 text-sm text-popover-foreground shadow-md">
      <p className="font-medium">{bar.tooltip}</p>
      <p>
        {getMoodEmoji(Math.round(bar.average))} {bar.average.toFixed(1)} avg
        <span className="ml-1 text-muted-foreground">
          ({bar.entryCount} {bar.entryCount === 1 ? "entry" : "entries"})
        </span>
      </p>
    </div>
  )
}
