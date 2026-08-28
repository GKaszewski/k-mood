import { lazy, Suspense } from "react"
import { Link, Outlet, createRootRoute } from "@tanstack/react-router"
import { buttonLook } from "@/lib/button-style"

/**
 * Devtools are development-only and lazily imported, so neither package is
 * reachable from the production entry chunk.
 */
const Devtools = import.meta.env.DEV
  ? lazy(() => import("@/components/dev/devtools"))
  : null

export const Route = createRootRoute({
  notFoundComponent: NotFound,
  component: RootLayout,
})

function NotFound() {
  return (
    <main className="flex min-h-svh items-center justify-center p-4">
      <div className="flex flex-col items-center gap-4 text-center">
        <h1 className="text-4xl font-bold">404</h1>
        <p className="text-muted-foreground">Page not found</p>
        <Link to="/" className={buttonLook({ variant: "outline" })}>
          Go home
        </Link>
      </div>
    </main>
  )
}

function RootLayout() {
  return (
    <>
      <Outlet />
      {Devtools && (
        <Suspense fallback={null}>
          <Devtools />
        </Suspense>
      )}
    </>
  )
}
