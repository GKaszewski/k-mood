import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import {
  previousPeriod,
  sameMonthLastYear,
  thisSeasonVsLastSeason,
  thisYearVsLastYear,
} from "@/lib/compare-presets"
import type { DaySpan, SpanPair } from "@/lib/compare-presets"

type PresetId = "month" | "year" | "season" | "previous"

interface Preset {
  id: PresetId
  label: string
  /** The pair this preset picks, given today, the requested B and the B actually compared. */
  pick: (today: Date, b: DaySpan, effectiveB: DaySpan) => SpanPair
}

const PRESETS: Preset[] = [
  {
    id: "month",
    label: "Same month last year",
    pick: (today) => sameMonthLastYear(today),
  },
  {
    id: "year",
    label: "This year vs last year",
    pick: (today) => thisYearVsLastYear(today),
  },
  {
    id: "season",
    label: "This season vs last season",
    pick: (today) => thisSeasonVsLastSeason(today),
  },
  {
    id: "previous",
    label: "Previous period",
    pick: (_, b, effectiveB) => [previousPeriod(effectiveB), b],
  },
]

function sameSpan(left: DaySpan, right: DaySpan): boolean {
  return left.from === right.from && left.to === right.to
}

interface PresetChipsProps {
  a: DaySpan
  b: DaySpan
  effectiveB?: DaySpan
  onPick: (pair: SpanPair) => void
}

/** Ready-made pairs. Picking one only fills in the two spans. */
export function PresetChips({ a, b, effectiveB, onPick }: PresetChipsProps) {
  const today = new Date()
  const comparedB = effectiveB ?? b
  const active = PRESETS.find((preset) => {
    const [presetA, presetB] = preset.pick(today, b, comparedB)
    return sameSpan(presetA, a) && sameSpan(presetB, b)
  })

  return (
    <ToggleGroup
      size="sm"
      variant="outline"
      className="flex-wrap"
      value={active ? [active.id] : []}
      onValueChange={(next: string[]) => {
        const preset = PRESETS.find((candidate) => candidate.id === next[0])
        if (preset) onPick(preset.pick(today, b, comparedB))
      }}
      aria-label="Comparison presets"
    >
      {PRESETS.map((preset) => (
        <ToggleGroupItem key={preset.id} value={preset.id}>
          {preset.label}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  )
}
