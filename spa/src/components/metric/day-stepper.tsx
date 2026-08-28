import { addDays, isToday, subDays } from "date-fns"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { formatDate, parseDate, today } from "@/lib/day"
import { describeDay } from "@/lib/day-label"

interface DayStepperProps {
  value: string
  onChange: (date: string) => void
}

/** Yesterday is one tap away, which is when most readings actually get typed. */
export function DayStepper({ value, onChange }: DayStepperProps) {
  const day = parseDate(value)
  const atToday = isToday(day)

  return (
    <div className="flex items-center gap-2">
      <Button
        variant="ghost"
        size="icon"
        aria-label="Previous day"
        onClick={() => onChange(formatDate(subDays(day, 1)))}
      >
        <ChevronLeft className="h-5 w-5" aria-hidden="true" />
      </Button>

      <div className="flex flex-1 flex-col items-center gap-1">
        <span className="text-sm font-medium">{describeDay(day)}</span>
        <Input
          type="date"
          aria-label="Day"
          className="h-8 text-center text-xs"
          value={value}
          max={today()}
          onChange={(event) => onChange(event.target.value)}
        />
      </div>

      <Button
        variant="ghost"
        size="icon"
        aria-label="Next day"
        disabled={atToday}
        onClick={() => onChange(formatDate(addDays(day, 1)))}
      >
        <ChevronRight className="h-5 w-5" aria-hidden="true" />
      </Button>
    </div>
  )
}
