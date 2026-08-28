import { Cell, Pie, PieChart, ResponsiveContainer } from "recharts"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { getMood } from "@/lib/mood"
import type { MoodFrequencyResponse } from "@/api/schema"

interface MoodDistributionProps {
  frequency: MoodFrequencyResponse[]
}

export function MoodDistribution({ frequency }: MoodDistributionProps) {
  if (!frequency.length) return null

  const data = frequency.map((entry) => ({
    label: entry.moodLabel,
    emoji: getMood(entry.mood).emoji,
    swatch: getMood(entry.mood).swatch,
    value: entry.count,
    mood: entry.mood,
  }))

  const total = data.reduce((sum, d) => sum + d.value, 0)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood distribution</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex items-center gap-4">
          <div aria-hidden="true">
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
                  {data.map((slice) => (
                    <Cell key={slice.mood} fill={slice.swatch} />
                  ))}
                </Pie>
              </PieChart>
            </ResponsiveContainer>
          </div>
          <ul className="flex list-none flex-col gap-1.5 p-0">
            {data.map((slice) => (
              <li key={slice.mood} className="flex items-center gap-2 text-sm">
                <span
                  aria-hidden="true"
                  className="h-2.5 w-2.5 shrink-0 rounded-full"
                  style={{ backgroundColor: slice.swatch }}
                />
                <span aria-hidden="true">{slice.emoji}</span>
                <span className="text-foreground">{slice.label}</span>
                <span className="text-foreground/70">
                  {slice.value} ({Math.round((slice.value / total) * 100)}%)
                </span>
              </li>
            ))}
          </ul>
        </div>
      </CardContent>
    </Card>
  )
}
