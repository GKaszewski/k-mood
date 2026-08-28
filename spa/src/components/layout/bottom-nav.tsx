import { Suspense, lazy, useState } from "react"
import { Link, useRouterState } from "@tanstack/react-router"
import { Home, Calendar, BarChart3, Settings, Plus } from "lucide-react"
import { cn } from "@/lib/utils"

/**
 * The form behind the button is the heaviest thing in the app — a date picker,
 * a drawer and every media control. None of it is needed until someone taps.
 */
const LogSheet = lazy(() =>
  import("@/components/entry/log-sheet").then((module) => ({
    default: module.LogSheet,
  }))
)

type NavItem = {
  to: string
  icon: typeof Home
  label: string
  /**
   * The routes this tab stands for. A tab stays lit while you are anywhere
   * beneath it — reading an entry belongs to Home, managing activities to
   * Settings — so no screen in the app is left with nothing selected.
   */
  owns: string[]
}

const NAV_ITEMS: NavItem[] = [
  {
    to: "/",
    icon: Home,
    label: "Home",
    owns: ["/", "/diary", "/entry", "/edit"],
  },
  { to: "/calendar", icon: Calendar, label: "Calendar", owns: ["/calendar"] },
  {
    to: "/stats",
    icon: BarChart3,
    label: "Stats",
    owns: ["/stats", "/metrics"],
  },
  {
    to: "/settings",
    icon: Settings,
    label: "Settings",
    owns: ["/settings", "/activities", "/reminders", "/import", "/export"],
  },
]

function owns(item: NavItem, pathname: string): boolean {
  return item.owns.some(
    (route) =>
      pathname === route || (route !== "/" && pathname.startsWith(`${route}/`))
  )
}

export function BottomNav() {
  const { location } = useRouterState()
  const [logging, setLogging] = useState(false)
  const [before, after] = [NAV_ITEMS.slice(0, 2), NAV_ITEMS.slice(2)]

  return (
    <>
      {logging && (
        <Suspense fallback={null}>
          <LogSheet open={logging} onOpenChange={setLogging} />
        </Suspense>
      )}
      <nav
        aria-label="Primary"
        className="fixed right-0 bottom-0 left-0 z-50 border-t bg-background/95 pb-safe backdrop-blur supports-[backdrop-filter]:bg-background/60"
      >
        <div className="mx-auto flex max-w-md items-center justify-around px-2 py-1">
          {before.map((item) => (
            <NavTab key={item.to} item={item} pathname={location.pathname} />
          ))}

          <button
            type="button"
            aria-label="Log a mood"
            aria-haspopup="dialog"
            onClick={() => setLogging(true)}
            className="-mt-5 flex h-14 w-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg"
          >
            <Plus className="h-6 w-6" aria-hidden="true" />
          </button>

          {after.map((item) => (
            <NavTab key={item.to} item={item} pathname={location.pathname} />
          ))}
        </div>
      </nav>
    </>
  )
}

function NavTab({ item, pathname }: { item: NavItem; pathname: string }) {
  const isActive = owns(item, pathname)
  const Icon = item.icon

  return (
    <Link
      to={item.to}
      aria-current={isActive ? "page" : undefined}
      className={cn(
        "flex flex-col items-center gap-0.5 px-3 py-2 text-xs transition-colors",
        isActive
          ? "text-primary"
          : "text-muted-foreground hover:text-foreground"
      )}
    >
      <Icon className="h-5 w-5" aria-hidden="true" />
      <span>{item.label}</span>
    </Link>
  )
}
