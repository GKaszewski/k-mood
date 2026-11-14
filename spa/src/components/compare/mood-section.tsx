import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ComparisonTable } from "@/components/compare/comparison-table"
import type { ComparisonLine } from "@/components/compare/comparison-row"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import { formatShare, kindOfRow } from "@/lib/compare-format"
import { getMood } from "@/lib/mood"
import type {
  MoodComparisonRowResponse,
  MoodSectionResponse,
} from "@/api/schema"

const PERCENT = 100

const LABELS: Record<string, string> = {
  averageMood: "Average mood",
  longestLoggingStreak: "Longest logging streak",
  longestGoodStreak: "Longest good streak",
}

function lineOf(row: MoodComparisonRowResponse): ComparisonLine {
  if (row.mood !== null) {
    const mood = getMood(row.mood)
    return {
      id: `moodShare:${row.mood}`,
      label: (
        <>
          <span aria-hidden="true">{mood.emoji} </span>
          {mood.label} days
        </>
      ),
      kind: kindOfRow(row.key),
      row,
      secondary: true,
    }
  }

  return {
    id: row.key,
    label: LABELS[row.key] ?? row.key,
    kind: kindOfRow(row.key),
    row,
  }
}

export function MoodSection({ section }: { section: MoodSectionResponse }) {
  const shares = section.rows.filter((row) => row.mood !== null)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Mood</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {shares.length > 0 && (
          <div className="flex flex-col gap-2">
            <Distribution side="A" color={PERIOD_A_COLOR} shares={shares} />
            <Distribution side="B" color={PERIOD_B_COLOR} shares={shares} />
          </div>
        )}
        <ComparisonTable lines={section.rows.map(lineOf)} />
      </CardContent>
    </Card>
  )
}

/** One period's logged days split by mood, as the server measured them. */
function Distribution({
  side,
  color,
  shares,
}: {
  side: "A" | "B"
  color: string
  shares: MoodComparisonRowResponse[]
}) {
  const parts = shares.map((row) => ({
    mood: getMood(row.mood ?? 0),
    share: (side === "A" ? row.a : row.b) ?? 0,
  }))
  const description = parts
    .map((part) => `${part.mood.label} ${formatShare(part.share)}`)
    .join(", ")

  return (
    <div className="flex items-center gap-2">
      <span className="w-6 shrink-0 text-xs font-medium" aria-hidden="true">
        <span style={{ color }}>●</span> {side}
      </span>
      <div
        role="img"
        aria-label={`Period ${side} days by mood: ${description}`}
        className="flex h-3 flex-1 overflow-hidden rounded-full bg-muted"
      >
        {parts.map((part) => (
          <span
            key={part.mood.value}
            className={part.mood.background}
            style={{ width: `${part.share * PERCENT}%` }}
          />
        ))}
      </div>
    </div>
  )
}
