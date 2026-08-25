import { Cell, Pie, PieChart, ResponsiveContainer } from "recharts"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { getMood } from "@/lib/mood"
import type { MoodFrequency } from "@/api/schema"

interface MoodDistributionProps {
  frequency: MoodFrequency[]
}

const COLORS: Record<number, string> = {
  1: "#ef4444",
  2: "#f97316",
  3: "#eab308",
  4: "#84cc16",
  5: "#22c55e",
}

export function MoodDistribution({ frequency }: MoodDistributionProps) {
  if (!frequency.length) return null

  const data = frequency.map((f) => ({
    name: getMood(f.mood).emoji + " " + f.moodLabel,
    value: f.count,
    mood: f.mood,
  }))

  const total = data.reduce((sum, d) => sum + d.value, 0)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood distribution</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex items-center gap-4">
          <ResponsiveContainer width={120} height={120}>
            <PieChart>
              <Pie
                data={data}
                dataKey="value"
                cx="50%"
                cy="50%"
                innerRadius={30}
                outerRadius={55}
                strokeWidth={2}
              >
                {data.map((d) => (
                  <Cell key={d.mood} fill={COLORS[d.mood]} />
                ))}
              </Pie>
            </PieChart>
          </ResponsiveContainer>
          <div className="flex flex-col gap-1.5">
            {data.map((d) => (
              <div key={d.mood} className="flex items-center gap-2 text-sm">
                <span
                  className="h-2.5 w-2.5 shrink-0 rounded-full"
                  style={{ backgroundColor: COLORS[d.mood] }}
                />
                <span className="text-foreground">{d.name}</span>
                <span className="text-foreground/70">
                  {d.value} ({Math.round((d.value / total) * 100)}%)
                </span>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
