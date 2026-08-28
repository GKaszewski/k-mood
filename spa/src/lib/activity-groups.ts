import type { ActivityResponse } from "@/api/schema"

/** Where activities with no category of their own are shown. */
export const UNCATEGORISED = "Other"

export interface ActivityGrouping<T> {
  category: string
  items: T[]
}

function byCategoryName(left: string, right: string): number {
  if (left === UNCATEGORISED) return 1
  if (right === UNCATEGORISED) return -1
  return left.localeCompare(right)
}

/**
 * Activities under their categories, uncategorised ones last. Used wherever
 * activities are listed, so one screen never orders them differently from
 * another.
 */
export function groupByCategory<T extends { category?: string | null }>(
  activities: T[]
): ActivityGrouping<T>[] {
  const groups = new Map<string, T[]>()

  for (const activity of activities) {
    const category = activity.category ?? UNCATEGORISED
    const existing = groups.get(category)

    if (existing) {
      existing.push(activity)
    } else {
      groups.set(category, [activity])
    }
  }

  return Array.from(groups.entries())
    .sort(([left], [right]) => byCategoryName(left, right))
    .map(([category, items]) => ({ category, items }))
}

/** The named activities behind a set of ids, grouped the same way. */
export function groupNamesById(
  activityIds: string[],
  catalogue: ActivityResponse[]
): ActivityGrouping<{ id: string; name: string }>[] {
  const known = new Map(catalogue.map((activity) => [activity.id, activity]))

  const named = activityIds.map((id) => {
    const activity = known.get(id)
    return {
      id,
      name: activity?.name ?? "Unknown activity",
      category: activity?.category ?? null,
    }
  })

  return groupByCategory(named)
}
