import type { ReactNode } from "react"
import { useMatches } from "@tanstack/react-router"
import { cn } from "@/lib/utils"
import { PrimaryNav } from "./primary-nav"

interface AppShellProps {
  children: ReactNode
}

/**
 * The phone column everywhere, and on a wide screen a side rail instead of the
 * bottom bar. Screens marked `wide` — the ones for reading data — then spread
 * across the room; the rest stay a readable column, since entries are logged
 * from the phone.
 */
export function AppShell({ children }: AppShellProps) {
  const wide = useMatches({
    select: (matches) => matches.some((match) => match.staticData.wide),
  })

  return (
    <div className="min-h-svh lg:pl-rail">
      <div
        className={cn(
          "mx-auto min-h-svh max-w-md app-column",
          wide && "lg:max-w-6xl"
        )}
      >
        <main className="px-4 pt-safe pb-nav lg:px-8 lg:pb-10">{children}</main>
      </div>
      <PrimaryNav />
    </div>
  )
}
