import type { WeekStartName } from "@/api/schema"

const MONDAY = 1

type WeekStart = 0 | 1 | 2 | 3 | 4 | 5 | 6

interface WeekInfo {
  firstDay: number
}

interface LocaleWithWeekInfo extends Intl.Locale {
  getWeekInfo?: () => WeekInfo
  weekInfo?: WeekInfo
}

/**
 * The day the reader's week starts on, as `Date.getDay` numbers it (Sunday 0).
 * The calendar used to be hardcoded to Sunday, which is wrong for most of the
 * world. Falls back to Monday rather than Sunday when the platform will not say.
 */
export function weekStartsOn(): WeekStart {
  try {
    const locale = new Intl.Locale(navigator.language) as LocaleWithWeekInfo
    const info = locale.getWeekInfo?.() ?? locale.weekInfo

    // Intl counts Monday as 1 through Sunday as 7; Date counts Sunday as 0.
    if (info) return (info.firstDay % 7) as WeekStart
  } catch {
    // Older engines do not expose week info.
  }

  return MONDAY
}

const WEEK_START_NAMES: WeekStartName[] = [
  "sunday",
  "monday",
  "tuesday",
  "wednesday",
  "thursday",
  "friday",
  "saturday",
]

/** The reader's first weekday by name, which is how the API takes it. */
export function weekStartName(): WeekStartName {
  return WEEK_START_NAMES[weekStartsOn()]
}

/** Weekday initials, rotated to start on the reader's first day. */
export function weekdayInitials(): string[] {
  const sundayFirst = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
  const start = weekStartsOn()

  return sundayFirst.map((_, index) => sundayFirst[(index + start) % 7])
}

/** Which row of a week-based grid a day belongs in. */
export function rowOfWeek(dayOfWeek: number): number {
  return (dayOfWeek - weekStartsOn() + 7) % 7
}
