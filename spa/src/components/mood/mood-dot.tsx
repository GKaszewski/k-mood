import { getMood } from "@/lib/mood"
import { cn } from "@/lib/utils"

interface MoodDotProps {
  value: number
  size?: "sm" | "md"
  /**
   * Whether the dot needs to name itself. False where the surrounding control
   * already says which mood this is, so it is not read out twice.
   */
  labelled?: boolean
}

const SIZES = { sm: "h-2 w-2", md: "h-3 w-3" } as const

export function MoodDot({ value, size = "sm", labelled = true }: MoodDotProps) {
  const mood = getMood(value)

  return (
    <span
      role={labelled ? "img" : undefined}
      aria-label={labelled ? mood.label : undefined}
      aria-hidden={labelled ? undefined : true}
      className={cn("inline-block rounded-full", SIZES[size], mood.background)}
    />
  )
}
