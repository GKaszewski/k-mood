import type { ComparisonRowResponse } from "@/api/schema"

/** Rows about one subject (an activity, a metric), its headline row first. */
export interface RowGroup<TRow extends ComparisonRowResponse> {
  id: string
  lead: TRow
  rows: TRow[]
}

export type GroupOrder = "server" | "change"

/**
 * Gathers rows by subject in the order the server sent them. The lead is the
 * row with `leadKey`, or the first row when the subject has none.
 */
export function groupRows<TRow extends ComparisonRowResponse>(
  rows: TRow[],
  subjectOf: (row: TRow) => string,
  leadKey: string
): RowGroup<TRow>[] {
  const groups = new Map<string, TRow[]>()
  for (const row of rows) {
    const id = subjectOf(row)
    groups.set(id, [...(groups.get(id) ?? []), row])
  }

  return [...groups].map(([id, members]) => {
    const lead = members.find((row) => row.key === leadKey) ?? members[0]
    return { id, lead, rows: [lead, ...members.filter((row) => row !== lead)] }
  })
}

/** Which change a "biggest change" order ranks by. */
export type ChangeMeasure = "absolute" | "relative"

/** Size of the lead row's change, or null when there is none to rank by. */
function sizeOfChange(
  row: ComparisonRowResponse,
  measure: ChangeMeasure
): number | null {
  const change =
    measure === "absolute" ? row.absoluteChange : row.relativeChange
  return change === null ? null : Math.abs(change)
}

/**
 * Largest change in the lead row first; groups with no change go last. Rows in
 * one unit (shares) can rank by absolute change; rows in different units
 * (steps beside heart rate) must rank by relative change.
 */
export function orderGroups<TRow extends ComparisonRowResponse>(
  groups: RowGroup<TRow>[],
  order: GroupOrder,
  measure: ChangeMeasure = "absolute"
): RowGroup<TRow>[] {
  if (order === "server") return groups
  return [...groups].sort((left, right) => {
    const l = sizeOfChange(left.lead, measure)
    const r = sizeOfChange(right.lead, measure)
    if (l === null) return r === null ? 0 : 1
    if (r === null) return -1
    return r - l
  })
}
