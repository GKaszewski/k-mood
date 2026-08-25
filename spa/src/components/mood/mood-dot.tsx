import { getMood } from "@/lib/mood"
import { cn } from "@/lib/utils"

interface MoodDotProps {
  value: number
  size?: "sm" | "md"
}

const BG_COLORS: Record<number, string> = {
  1: "bg-red-500",
  2: "bg-orange-500",
  3: "bg-yellow-500",
  4: "bg-lime-500",
  5: "bg-green-500",
}

export function MoodDot({ value, size = "sm" }: MoodDotProps) {
  const sizes = { sm: "h-2 w-2", md: "h-3 w-3" }

  return (
    <span
      className={cn("inline-block rounded-full", sizes[size], BG_COLORS[value])}
      title={getMood(value).label}
    />
  )
}
