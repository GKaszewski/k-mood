import type { ReactNode } from "react"
import { TableCell, TableRow } from "@/components/ui/table"
import { ChangeBar } from "@/components/compare/change-bar"
import { SignificanceBadge } from "@/components/compare/significance-badge"
import { formatChange, formatRelative, formatValue } from "@/lib/compare-format"
import type { ValueKind } from "@/lib/compare-format"
import type { ComparisonRowResponse } from "@/api/schema"
import { cn } from "@/lib/utils"

/** One figure as a table line: what it is called and how it is written. */
export interface ComparisonLine {
  id: string
  label: ReactNode
  kind: ValueKind
  row: ComparisonRowResponse
  /** A supporting figure under a primary one, set in a quieter style. */
  secondary?: boolean
}

export function ComparisonRow({ line }: { line: ComparisonLine }) {
  const { row, kind } = line

  return (
    <TableRow>
      <TableCell
        className={cn(
          "max-w-40 truncate",
          line.secondary && "pl-5 text-xs text-muted-foreground"
        )}
      >
        {line.label}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {formatValue(kind, row.a)}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {formatValue(kind, row.b)}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        <span>{formatChange(kind, row.absoluteChange)}</span>
        {row.relativeChange !== null && (
          <span className="ml-1 text-[10px] text-muted-foreground">
            {formatRelative(row.relativeChange)}
          </span>
        )}
      </TableCell>
      <TableCell className="w-16">
        <ChangeBar
          kind={kind}
          absoluteChange={row.absoluteChange}
          relativeChange={row.relativeChange}
        />
      </TableCell>
      <TableCell className="text-right">
        <SignificanceBadge
          significance={row.significance}
          sampleA={row.sampleA}
          sampleB={row.sampleB}
        />
      </TableCell>
    </TableRow>
  )
}
