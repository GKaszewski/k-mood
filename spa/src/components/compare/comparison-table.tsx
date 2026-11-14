import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { ComparisonRow } from "@/components/compare/comparison-row"
import type { ComparisonLine } from "@/components/compare/comparison-row"
import { PERIOD_A_COLOR, PERIOD_B_COLOR } from "@/lib/compare-colors"

interface ComparisonTableProps {
  lines: ComparisonLine[]
  /** What the label column holds, e.g. "Activity". */
  subject?: string
}

export function ComparisonTable({
  lines,
  subject = "Figure",
}: ComparisonTableProps) {
  return (
    <Table className="text-sm">
      <TableHeader>
        <TableRow>
          <TableHead>{subject}</TableHead>
          <TableHead className="text-right">
            <span aria-hidden="true" style={{ color: PERIOD_A_COLOR }}>
              ●{" "}
            </span>
            A
          </TableHead>
          <TableHead className="text-right">
            <span aria-hidden="true" style={{ color: PERIOD_B_COLOR }}>
              ●{" "}
            </span>
            B
          </TableHead>
          <TableHead className="text-right">
            <span aria-hidden="true">Δ</span>
            <span className="sr-only">Change from A to B</span>
          </TableHead>
          <TableHead>
            <span className="sr-only">Which period is higher</span>
          </TableHead>
          <TableHead>
            <span className="sr-only">Could it be chance</span>
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {lines.map((line) => (
          <ComparisonRow key={line.id} line={line} />
        ))}
      </TableBody>
    </Table>
  )
}
