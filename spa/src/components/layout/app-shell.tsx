import type { ReactNode } from "react"
import { BottomNav } from "./bottom-nav"

interface AppShellProps {
  children: ReactNode
}

export function AppShell({ children }: AppShellProps) {
  return (
    <div className="mx-auto min-h-svh max-w-md bg-background">
      <main className="pt-safe px-4 pb-20">{children}</main>
      <BottomNav />
    </div>
  )
}
