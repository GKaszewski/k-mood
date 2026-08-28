import { createRouter } from "@tanstack/react-router"
import { routeTree } from "./routeTree.gen"
import { RouteError } from "@/components/layout/route-error"

export function getRouter() {
  return createRouter({
    routeTree,
    scrollRestoration: true,
    defaultPreload: "intent",
    // Without this a single failing screen unmounts the whole application.
    defaultErrorComponent: RouteError,
  })
}

declare module "@tanstack/react-router" {
  interface Register {
    router: ReturnType<typeof getRouter>
  }
}
