import type { ReactNode } from "react"
import { BottomNav } from "./bottom-nav"

interface AppShellProps {
  children: ReactNode
}

export function AppShell({ children }: AppShellProps) {
  return (
    <div className="mx-auto min-h-svh max-w-md app-column">
      <main className="px-4 pt-safe pb-nav">{children}</main>
      <BottomNav />
    </div>
  )
}
