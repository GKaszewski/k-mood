import { useState } from "react"
import type { DateRange } from "react-day-picker"
import { Button, buttonVariants } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { formatDate, parseDate } from "@/lib/day"
import { formatSpan } from "@/lib/compare-format"
import type { DaySpan } from "@/lib/compare-presets"
import type { PeriodResponse } from "@/api/schema"
import { cn } from "@/lib/utils"

interface PeriodPickerProps {
  side: "A" | "B"
  color: string
  span: DaySpan
  /** The span as the server measured it, once known: clipped to today. */
  effective?: PeriodResponse
  onChange: (span: DaySpan) => void
}

function rangeOf(span: DaySpan): DateRange {
  return { from: parseDate(span.from), to: parseDate(span.to) }
}

export function PeriodPicker({
  side,
  color,
  span,
  effective,
  onChange,
}: PeriodPickerProps) {
  const [open, setOpen] = useState(false)
  const [draft, setDraft] = useState<DateRange | undefined>(rangeOf(span))

  const clippedTo = effective?.clipped ? effective.to : undefined
  const label = formatSpan(span.from, clippedTo ?? span.to, Boolean(clippedTo))

  const openChanged = (next: boolean) => {
    if (next) setDraft(rangeOf(span))
    setOpen(next)
  }

  const apply = () => {
    if (!draft?.from) return
    const to = draft.to ?? draft.from
    onChange({ from: formatDate(draft.from), to: formatDate(to) })
    setOpen(false)
  }

  return (
    <Popover open={open} onOpenChange={openChanged}>
      <PopoverTrigger
        className={cn(
          buttonVariants({ variant: "outline", size: "sm" }),
          "justify-start font-normal"
        )}
        aria-label={`Period ${side}: ${label}. Change`}
      >
        <span aria-hidden="true" style={{ color }}>
          ●
        </span>
        <span className="font-medium">{side}</span>
        <span className="truncate">{label}</span>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-auto p-0">
        <Calendar
          mode="range"
          selected={draft}
          defaultMonth={draft?.from}
          onSelect={setDraft}
          autoFocus
        />
        <div className="flex justify-end gap-2 border-t p-2">
          <Button variant="ghost" size="sm" onClick={() => setOpen(false)}>
            Cancel
          </Button>
          <Button size="sm" disabled={!draft?.from} onClick={apply}>
            Compare
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  )
}
