import { createFileRoute } from "@tanstack/react-router"

export const Route = createFileRoute("/_auth/stats")({
  staticData: { wide: true },
})
