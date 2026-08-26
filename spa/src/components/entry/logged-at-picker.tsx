import { useState } from "react"
import { format } from "date-fns"
import { CalendarIcon } from "lucide-react"

import { buttonVariants } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import { Input } from "@/components/ui/input"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { cn } from "@/lib/utils"
import { timeOfDayOf, withDayOf, withTimeOfDay } from "@/lib/logged-at"

const DAY_LABEL = "d MMM yyyy"

interface LoggedAtPickerProps {
  value: Date
  onChange: (next: Date) => void
}

export function LoggedAtPicker({ value, onChange }: LoggedAtPickerProps) {
  const [calendarOpen, setCalendarOpen] = useState(false)

  const pickDay = (day: Date | undefined) => {
    if (!day) return
    onChange(withDayOf(value, day))
    setCalendarOpen(false)
  }

  return (
    <div className="flex items-center gap-2">
      <Popover open={calendarOpen} onOpenChange={setCalendarOpen}>
        <PopoverTrigger
          className={cn(
            buttonVariants({ variant: "outline", size: "sm" }),
            "font-normal"
          )}
        >
          <CalendarIcon />
          {format(value, DAY_LABEL)}
        </PopoverTrigger>
        <PopoverContent align="start" className="w-auto p-0">
          <Calendar
            mode="single"
            selected={value}
            defaultMonth={value}
            onSelect={pickDay}
            autoFocus
          />
        </PopoverContent>
      </Popover>

      <Input
        type="time"
        className="w-28"
        value={timeOfDayOf(value)}
        onChange={(event) => onChange(withTimeOfDay(value, event.target.value))}
      />
    </div>
  )
}
