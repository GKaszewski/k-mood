import { format, startOfWeek, startOfMonth } from "date-fns"
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
import { getMoodEmoji } from "@/lib/mood"

interface MoodTrendChartProps {
  entries: Array<{ loggedAt: string; mood: number }>
  period?: "week" | "month" | "year" | "all"
  title?: string
}

export function MoodTrendChart({
  entries,
  period = "month",
  title = "Mood over time",
}: MoodTrendChartProps) {
  const data = aggregate(entries, period)

  if (data.length === 0) return null

  const showDots = data.length <= 60

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={220}>
          <AreaChart
            data={data}
            margin={{ top: 5, right: 5, bottom: 0, left: -10 }}
          >
            <defs>
              <linearGradient id="moodGradient" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="#22c55e" stopOpacity={0.4} />
                <stop offset="50%" stopColor="#eab308" stopOpacity={0.2} />
                <stop offset="100%" stopColor="#ef4444" stopOpacity={0.05} />
              </linearGradient>
            </defs>
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="hsl(var(--border))"
              vertical={false}
            />
            <XAxis
              dataKey="label"
              tick={{ fontSize: 11, fill: "white" }}
              tickLine={false}
              axisLine={false}
              interval="preserveStartEnd"
              minTickGap={50}
            />
            <YAxis
              domain={[0.5, 5.5]}
              ticks={[1, 2, 3, 4, 5]}
              tick={{ fontSize: 14 }}
              tickFormatter={(v) => getMoodEmoji(v)}
              tickLine={false}
              axisLine={false}
              width={32}
            />
            <Tooltip
              content={({ active, payload }) => {
                if (!active || !payload?.length) return null
                const d = payload[0].payload
                const mood = Math.round(d.avg)
                return (
                  <div className="rounded-lg border bg-background px-3 py-2 text-sm shadow-md">
                    <p className="font-medium">{d.tooltip}</p>
                    <p>
                      {getMoodEmoji(mood)} {d.avg.toFixed(1)} avg
                      <span className="ml-1 text-muted-foreground">
                        ({d.count} {d.count === 1 ? "entry" : "entries"})
                      </span>
                    </p>
                  </div>
                )
              }}
            />
            <Area
              type="natural"
              dataKey="avg"
              stroke="#22c55e"
              fill="url(#moodGradient)"
              strokeWidth={2}
              dot={
                showDots
                  ? {
                      r: 2.5,
                      fill: "hsl(var(--background))",
                      stroke: "#22c55e",
                      strokeWidth: 1.5,
                    }
                  : false
              }
              activeDot={{ r: 4, fill: "#22c55e" }}
            />
          </AreaChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

interface DataPoint {
  label: string
  tooltip: string
  avg: number
  count: number
}

function aggregate(
  entries: Array<{ loggedAt: string; mood: number }>,
  period: string
): DataPoint[] {
  if (period === "year" || period === "all") {
    return entries.length > 365
      ? aggregateBy(entries, "month")
      : aggregateBy(entries, "week")
  }
  return aggregateBy(entries, "day")
}

function aggregateBy(
  entries: Array<{ loggedAt: string; mood: number }>,
  bucket: "day" | "week" | "month"
): DataPoint[] {
  const groups = new Map<
    string,
    { moods: number[]; label: string; tooltip: string }
  >()

  for (const e of entries) {
    const d = new Date(e.loggedAt)
    let key: string
    let label: string
    let tooltip: string

    if (bucket === "month") {
      key = format(startOfMonth(d), "yyyy-MM")
      label = format(d, "MMM yyyy")
      tooltip = format(d, "MMMM yyyy")
    } else if (bucket === "week") {
      const week = startOfWeek(d)
      key = format(week, "yyyy-MM-dd")
      label = format(week, "MMM d")
      tooltip = `Week of ${format(week, "MMM d, yyyy")}`
    } else {
      key = format(d, "yyyy-MM-dd")
      label = format(d, "MMM d")
      tooltip = format(d, "EEE, MMM d")
    }

    if (!groups.has(key)) {
      groups.set(key, { moods: [], label, tooltip })
    }
    groups.get(key)!.moods.push(e.mood)
  }

  return Array.from(groups.entries())
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([_, { moods, label, tooltip }]) => ({
      label,
      tooltip,
      avg: moods.reduce((s, m) => s + m, 0) / moods.length,
      count: moods.length,
    }))
}
