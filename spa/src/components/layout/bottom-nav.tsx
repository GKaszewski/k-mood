import { Link, useRouterState } from "@tanstack/react-router"
import { Home, Calendar, BarChart3, Settings, Plus } from "lucide-react"
import { cn } from "@/lib/utils"

type NavItem = {
  to: string
  icon: typeof Home
  label: string
  primary?: boolean
}

const NAV_ITEMS: NavItem[] = [
  { to: "/", icon: Home, label: "Home" },
  { to: "/calendar", icon: Calendar, label: "Calendar" },
  { to: "/add", icon: Plus, label: "Add", primary: true },
  { to: "/stats", icon: BarChart3, label: "Stats" },
  { to: "/settings", icon: Settings, label: "Settings" },
]

export function BottomNav() {
  const { location } = useRouterState()

  return (
    <nav className="safe-area-pb fixed right-0 bottom-0 left-0 z-50 border-t bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="mx-auto flex max-w-md items-center justify-around px-2 py-1">
        {NAV_ITEMS.map((item) => {
          const isActive = location.pathname === item.to
          const Icon = item.icon

          if (item.primary) {
            return (
              <Link
                key={item.to}
                to={item.to}
                className="-mt-5 flex h-14 w-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg"
              >
                <Icon className="h-6 w-6" />
              </Link>
            )
          }

          return (
            <Link
              key={item.to}
              to={item.to}
              className={cn(
                "flex flex-col items-center gap-0.5 px-3 py-2 text-xs transition-colors",
                isActive
                  ? "text-primary"
                  : "text-muted-foreground hover:text-foreground"
              )}
            >
              <Icon className="h-5 w-5" />
              <span>{item.label}</span>
            </Link>
          )
        })}
      </div>
    </nav>
  )
}
