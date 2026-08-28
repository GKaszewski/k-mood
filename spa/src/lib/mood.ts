/**
 * The five moods, in order. One place for every way a mood is shown: `color`
 * for text, `background` for a filled cell, `swatch` for canvases and charts
 * that cannot take a class. They were four separate maps that had drifted.
 */
export const MOODS = [
  {
    value: 1,
    label: "Awful",
    emoji: "😢",
    color: "text-red-500",
    background: "bg-red-500",
    swatch: "oklch(0.637 0.237 25.331)",
  },
  {
    value: 2,
    label: "Bad",
    emoji: "😔",
    color: "text-orange-500",
    background: "bg-orange-500",
    swatch: "oklch(0.705 0.213 47.604)",
  },
  {
    value: 3,
    label: "Meh",
    emoji: "😐",
    color: "text-yellow-500",
    background: "bg-yellow-500",
    swatch: "oklch(0.795 0.184 86.047)",
  },
  {
    value: 4,
    label: "Good",
    emoji: "😊",
    color: "text-lime-500",
    background: "bg-lime-500",
    swatch: "oklch(0.768 0.233 130.85)",
  },
  {
    value: 5,
    label: "Rad",
    emoji: "😄",
    color: "text-green-500",
    background: "bg-green-500",
    swatch: "oklch(0.723 0.219 149.579)",
  },
] as const

export type MoodValue = (typeof MOODS)[number]["value"]

export const WORST_MOOD = MOODS[0]
export const MIDDLE_MOOD = MOODS[2]
export const BEST_MOOD = MOODS[MOODS.length - 1]

export function getMood(value: number) {
  return MOODS.find((mood) => mood.value === value) ?? MIDDLE_MOOD
}

export function getMoodColor(value: number): string {
  return getMood(value).color
}

export function getMoodEmoji(value: number): string {
  return getMood(value).emoji
}

export function getMoodLabel(value: number): string {
  return getMood(value).label
}

/**
 * How a mood reads to a screen reader. Emoji and colour carry the mood
 * everywhere in the UI; this is the same information in words.
 */
export function describeMood(value: number): string {
  return `Mood: ${getMoodLabel(value)}`
}

/** Rounds an average onto the scale, for a label that has to name one mood. */
export function nearestMood(average: number) {
  const clamped = Math.min(BEST_MOOD.value, Math.max(WORST_MOOD.value, average))
  return getMood(Math.round(clamped))
}
