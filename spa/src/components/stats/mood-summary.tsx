import { StatCard } from "./stat-card"
import { nearestMood } from "@/lib/mood"
import type { MoodStatsResponse } from "@/api/schema"

const NO_VALUE = "—"

/**
 * Average, streak and total, shown the same way wherever they appear — the
 * dashboard and the statistics page were drifting into two different readings
 * of the same three numbers.
 */
export function MoodSummary({ stats }: { stats: MoodStatsResponse }) {
  const mood = stats.average === null ? null : nearestMood(stats.average)

  return (
    <div className="grid grid-cols-3 gap-3">
      <StatCard
        label="Average"
        value={
          mood ? (
            <span className="inline-flex items-baseline gap-1">
              <span aria-hidden="true">{mood.emoji}</span>
              <span>{stats.average?.toFixed(1)}</span>
            </span>
          ) : (
            NO_VALUE
          )
        }
        valueLabel={
          mood
            ? `Average mood ${stats.average?.toFixed(1)} out of 5, ${mood.label}`
            : "No average yet"
        }
      />
      <StatCard
        label="Streak"
        value={`${stats.currentStreak}d`}
        valueLabel={`${stats.currentStreak} day streak`}
      />
      <StatCard label="Entries" value={stats.totalEntries} />
    </div>
  )
}
