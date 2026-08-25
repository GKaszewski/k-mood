import { MOODS, type MoodValue } from "@/lib/mood"
import { cn } from "@/lib/utils"

interface MoodPickerProps {
  value?: MoodValue
  onChange: (mood: MoodValue) => void
  size?: "sm" | "md" | "lg"
}

export function MoodPicker({ value, onChange, size = "md" }: MoodPickerProps) {
  const sizes = {
    sm: "text-2xl p-2",
    md: "text-4xl p-3",
    lg: "text-5xl p-4",
  }

  return (
    <div className="flex items-center justify-center gap-2">
      {MOODS.map((mood) => (
        <button
          key={mood.value}
          type="button"
          onClick={() => onChange(mood.value)}
          className={cn(
            "rounded-xl transition-all duration-200",
            sizes[size],
            value === mood.value
              ? "scale-110 ring-2 ring-primary"
              : "opacity-50 hover:scale-105 hover:opacity-100"
          )}
          aria-label={mood.label}
        >
          {mood.emoji}
        </button>
      ))}
    </div>
  )
}
