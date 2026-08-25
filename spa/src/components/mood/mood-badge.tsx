import { getMood } from "@/lib/mood"
import { cn } from "@/lib/utils"

interface MoodBadgeProps {
  value: number
  showLabel?: boolean
  className?: string
}

export function MoodBadge({
  value,
  showLabel = false,
  className,
}: MoodBadgeProps) {
  const mood = getMood(value)

  return (
    <span className={cn("inline-flex items-center gap-1", className)}>
      <span className="text-lg">{mood.emoji}</span>
      {showLabel && (
        <span className={cn("text-sm font-medium", mood.color)}>
          {mood.label}
        </span>
      )}
    </span>
  )
}
