export const MOODS = [
  { value: 1, label: "Awful", emoji: "😢", color: "text-red-500" },
  { value: 2, label: "Bad", emoji: "😔", color: "text-orange-500" },
  { value: 3, label: "Meh", emoji: "😐", color: "text-yellow-500" },
  { value: 4, label: "Good", emoji: "😊", color: "text-lime-500" },
  { value: 5, label: "Rad", emoji: "😄", color: "text-green-500" },
] as const

export type MoodValue = (typeof MOODS)[number]["value"]

export function getMood(value: number) {
  return MOODS.find((m) => m.value === value) ?? MOODS[2]
}

export function getMoodColor(value: number): string {
  return getMood(value).color
}

export function getMoodEmoji(value: number): string {
  return getMood(value).emoji
}
