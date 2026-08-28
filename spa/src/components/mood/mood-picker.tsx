import { useRef } from "react"
import { MOODS } from "@/lib/mood"
import type { MoodValue } from "@/lib/mood"
import { cn } from "@/lib/utils"

interface MoodPickerProps {
  value?: MoodValue
  onChange: (mood: MoodValue) => void
  size?: "sm" | "md" | "lg"
}

const SIZES = {
  sm: "text-2xl p-2",
  md: "text-4xl p-3",
  lg: "text-5xl p-4",
} as const

/**
 * A radio group, because that is what choosing one of five is. Arrow keys move
 * between the faces and a single tab stop enters the group, so the picker is
 * one control rather than five loose buttons.
 */
export function MoodPicker({ value, onChange, size = "md" }: MoodPickerProps) {
  const groupRef = useRef<HTMLDivElement>(null)

  const focusAt = (index: number) => {
    const options =
      groupRef.current?.querySelectorAll<HTMLButtonElement>("[role='radio']")
    options?.[(index + MOODS.length) % MOODS.length]?.focus()
  }

  const handleKey = (event: React.KeyboardEvent, index: number) => {
    const step =
      event.key === "ArrowRight" || event.key === "ArrowDown"
        ? 1
        : event.key === "ArrowLeft" || event.key === "ArrowUp"
          ? -1
          : 0

    if (step === 0) return

    event.preventDefault()
    const next = (index + step + MOODS.length) % MOODS.length
    onChange(MOODS[next].value)
    focusAt(next)
  }

  const focusedIndex = value
    ? MOODS.findIndex((mood) => mood.value === value)
    : 0

  return (
    <div
      ref={groupRef}
      role="radiogroup"
      aria-label="How are you feeling?"
      className="flex items-center justify-center gap-2"
    >
      {MOODS.map((mood, index) => {
        const isSelected = value === mood.value

        return (
          <button
            key={mood.value}
            type="button"
            role="radio"
            aria-checked={isSelected}
            aria-label={mood.label}
            tabIndex={index === focusedIndex ? 0 : -1}
            onClick={() => onChange(mood.value)}
            onKeyDown={(event) => handleKey(event, index)}
            className={cn(
              "rounded-xl transition-all duration-200",
              "focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
              SIZES[size],
              isSelected
                ? "scale-110 ring-2 ring-primary"
                : "opacity-60 hover:scale-105 hover:opacity-100"
            )}
          >
            <span aria-hidden="true">{mood.emoji}</span>
          </button>
        )
      })}
    </div>
  )
}
