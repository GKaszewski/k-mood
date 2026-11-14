import { Badge } from "@/components/ui/badge"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import type { Significance } from "@/api/schema"

type Tested = Exclude<Significance, "notTested">

const LOOKS: Record<
  Tested,
  {
    label: string
    variant: "default" | "secondary" | "outline"
    explains: string
  }
> = {
  likelyReal: {
    label: "likely real",
    variant: "default",
    explains:
      "Still stands once the number of rows compared is accounted for. Worth a look, not proof.",
  },
  noise: {
    label: "noise",
    variant: "secondary",
    explains:
      "A difference this size turns up by chance often enough that it may mean nothing.",
  },
  notEnoughData: {
    label: "not enough data",
    variant: "outline",
    explains: "Too few days, or too few occurrences, to test the difference.",
  },
}

interface SignificanceBadgeProps {
  significance: Significance
  sampleA: number
  sampleB: number
}

/** What the test made of a change. Descriptive rows are never tested and show nothing. */
export function SignificanceBadge({
  significance,
  sampleA,
  sampleB,
}: SignificanceBadgeProps) {
  if (significance === "notTested") return null

  const look = LOOKS[significance]
  const samples = `Based on ${sampleA} days in A and ${sampleB} days in B.`

  return (
    <Tooltip>
      <TooltipTrigger
        className="rounded-4xl focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-none"
        aria-label={`${look.label}: ${look.explains} ${samples}`}
      >
        <Badge variant={look.variant}>{look.label}</Badge>
      </TooltipTrigger>
      <TooltipContent className="flex-col items-start">
        <span>{look.explains}</span>
        <span>{samples}</span>
      </TooltipContent>
    </Tooltip>
  )
}
