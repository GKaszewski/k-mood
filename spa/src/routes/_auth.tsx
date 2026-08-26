import { createFileRoute, Outlet, redirect } from "@tanstack/react-router"
import { AppShell } from "@/components/layout/app-shell"
import { Skeleton } from "@/components/ui/skeleton"
import { useEnsureTimezone } from "@/hooks/use-timezone"

export const Route = createFileRoute("/_auth")({
  beforeLoad: () => {
    const token = localStorage.getItem("accessToken")
    if (!token) {
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

  return (
    <AppShell>
      <Outlet />
    </AppShell>
  )
}
