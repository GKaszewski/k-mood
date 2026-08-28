import { createFileRoute, Outlet, redirect } from "@tanstack/react-router"
import { AppShell } from "@/components/layout/app-shell"
import { ConnectionBanner } from "@/components/offline/connection-banner"
import { Skeleton } from "@/components/ui/skeleton"
import { hasSession } from "@/api/session"
import { useEnsureTimezone } from "@/hooks/use-timezone"
import { useOutboxSync } from "@/hooks/use-outbox"

export const Route = createFileRoute("/_auth")({
  beforeLoad: () => {
    if (!hasSession()) {
      throw redirect({ to: "/login" })
    }
  },
  component: AuthLayout,
  pendingComponent: () => (
    <AppShell>
      <div className="flex flex-col gap-4 py-6">
        <Skeleton className="h-8 w-32" />
        <Skeleton className="h-48" />
        <Skeleton className="h-32" />
      </div>
    </AppShell>
  ),
})

function AuthLayout() {
  useEnsureTimezone()
  useOutboxSync()

  return (
    <AppShell>
      <div className="pt-3">
        <ConnectionBanner />
      </div>
      <Outlet />
    </AppShell>
  )
}
