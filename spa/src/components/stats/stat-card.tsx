import { Card, CardContent } from "@/components/ui/card"
import type { ReactNode } from "react"

interface StatCardProps {
  label: string
  value: string | number
  icon?: ReactNode
}

export function StatCard({ label, value, icon }: StatCardProps) {
  return (
    <Card>
      <CardContent className="flex flex-col items-center gap-1 py-4">
        {icon && <span className="text-muted-foreground">{icon}</span>}
        <span className="text-2xl font-bold">{value}</span>
        <span className="text-xs text-muted-foreground">{label}</span>
      </CardContent>
    </Card>
  )
}
