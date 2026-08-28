import type { ReactNode } from "react"
import { Card, CardContent } from "@/components/ui/card"

interface StatCardProps {
  label: string
  value: ReactNode
  /** Read instead of `value` by assistive technology, when the value is a glyph. */
  valueLabel?: string
  icon?: ReactNode
}

export function StatCard({ label, value, valueLabel, icon }: StatCardProps) {
  return (
    <Card>
      <CardContent className="flex flex-col items-center gap-1 py-4">
        {icon && <span className="text-muted-foreground">{icon}</span>}
        <span
          className="text-2xl font-bold"
          aria-label={valueLabel}
          role={valueLabel ? "img" : undefined}
        >
          {value}
        </span>
        <span className="text-xs text-muted-foreground">{label}</span>
      </CardContent>
    </Card>
  )
}
