import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"
import type { ValueKind } from "@/lib/compare-format"

/** A change this large, or larger, fills half the bar. */
const FULL_SHARE_CHANGE = 0.5
const FULL_RELATIVE_CHANGE = 1
const HALF = 50

interface ChangeBarProps {
  kind: ValueKind
  absoluteChange: number | null
  relativeChange: number | null
}

/**
 * A diverging bar from the centre: right and blue when B is higher, left and
 * green when A is. It says which side is higher and roughly by how much, and
 * makes no claim about which is better. It repeats the Δ column, so it is
 * hidden from screen readers.
 */
export function ChangeBar({
  kind,
  absoluteChange,
  relativeChange,
}: ChangeBarProps) {
  const extent =
    kind === "share"
      ? absoluteChange !== null
        ? absoluteChange / FULL_SHARE_CHANGE
        : null
      : relativeChange !== null
        ? relativeChange / FULL_RELATIVE_CHANGE
        : null

  return (
    <div
      className="relative h-1.5 w-full min-w-12 rounded-full bg-muted"
      aria-hidden="true"
    >
      <span className="absolute inset-y-0 left-1/2 w-px bg-border" />
      {extent !== null && extent !== 0 && (
        <span
          className="absolute inset-y-0 rounded-full"
          style={{
            width: `${Math.min(Math.abs(extent), 1) * HALF}%`,
            ...(extent > 0 ? { left: "50%" } : { right: "50%" }),
            background: extent > 0 ? PERIOD_B_COLOR : PERIOD_A_COLOR,
          }}
        />
      )}
    </div>
  )
}
