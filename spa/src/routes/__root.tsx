import { Outlet, createRootRoute } from "@tanstack/react-router"
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

export const Route = createRootRoute({
  notFoundComponent: () => (
    <main className="flex min-h-svh items-center justify-center p-4">
      <div className="text-center">
        <h1 className="text-4xl font-bold">404</h1>
        <p className="mt-2 text-muted-foreground">Page not found</p>
      </div>
    </main>
  ),
  component: RootLayout,
})

function RootLayout() {
  return (
    <>
      <Outlet />
      <ReactQueryDevtools buttonPosition="bottom-left" />
      <TanStackRouterDevtools position="bottom-right" />
    </>
  )
}
