import type { ReactNode } from "react"
import { ChevronDown } from "lucide-react"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"

export function CollapsibleCard({
  title,
  description,
  summary,
  defaultOpen = false,
  children,
}: {
  title: string
  description?: string
  summary?: ReactNode
  defaultOpen?: boolean
  children: ReactNode
}) {
  return (
    <Card>
      <Collapsible defaultOpen={defaultOpen}>
        <CardHeader>
          <CollapsibleTrigger className="group flex w-full cursor-pointer items-center justify-between gap-3 text-left">
            <div className="flex min-w-0 flex-col gap-1">
              <CardTitle className="text-sm">{title}</CardTitle>
              {description && <CardDescription>{description}</CardDescription>}
            </div>
            <div className="flex shrink-0 items-center gap-2">
              {summary}
              <ChevronDown className="size-4 text-muted-foreground transition-transform duration-200 group-data-[panel-open]:rotate-180" />
            </div>
          </CollapsibleTrigger>
        </CardHeader>
        <CollapsibleContent className="h-[var(--collapsible-panel-height)] overflow-hidden transition-[height] duration-200 data-[ending-style]:h-0 data-[starting-style]:h-0">
          <CardContent>{children}</CardContent>
        </CollapsibleContent>
      </Collapsible>
    </Card>
  )
}
