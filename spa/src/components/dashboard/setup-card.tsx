import { Link } from "@tanstack/react-router"
import { Bell, ListChecks } from "lucide-react"
import type { LucideIcon } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useActivities } from "@/hooks/use-activities"
import { useReminders } from "@/hooks/use-reminders"
import { usePushNotifications } from "@/hooks/use-push"

/** Past this many entries the account is established, and nudging is nagging. */
const STILL_NEW_BELOW = 5

interface SetupStep {
  to: "/activities" | "/reminders"
  icon: LucideIcon
  label: string
  describes: string
}

/**
 * The two things that are worth setting up early and are otherwise only
 * reachable through Settings. Disappears on its own once they are done, or
 * once the account is no longer new.
 */
export function SetupCard({ totalEntries }: { totalEntries: number }) {
  const { data: activities } = useActivities()
  const { data: reminders } = useReminders()
  const push = usePushNotifications()

  if (totalEntries >= STILL_NEW_BELOW) return null

  const steps: SetupStep[] = []

  if (activities?.length === 0) {
    steps.push({
      to: "/activities",
      icon: ListChecks,
      label: "Add your activities",
      describes: "What you want to tag entries with",
    })
  }

  if (push.isSupported && push.hasVapidKey && reminders?.length === 0) {
    steps.push({
      to: "/reminders",
      icon: Bell,
      label: "Set a reminder",
      describes: "A nudge at the time you want to check in",
    })
  }

  if (steps.length === 0) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Finish setting up</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-1 p-0 pb-2">
        {steps.map((step) => {
          const Icon = step.icon

          return (
            <Link
              key={step.to}
              to={step.to}
              className="flex items-center gap-3 px-6 py-3 transition-colors hover:bg-muted/50"
            >
              <Icon
                className="h-4 w-4 shrink-0 text-muted-foreground"
                aria-hidden="true"
              />
              <span className="flex flex-col">
                <span className="text-sm">{step.label}</span>
                <span className="text-xs text-muted-foreground">
                  {step.describes}
                </span>
              </span>
            </Link>
          )
        })}
      </CardContent>
    </Card>
  )
}
