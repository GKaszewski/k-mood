import { createFileRoute } from "@tanstack/react-router"
import { ComparedSpansParams } from "@/api/schema"

const CompareSearch = ComparedSpansParams.refine(
  (search) => search.aFrom <= search.aTo && search.bFrom <= search.bTo
)

export type CompareSearch = Partial<ComparedSpansParams>

/**
 * Both spans live in the URL so a comparison can be bookmarked and shared. A
 * partial, malformed or reversed set is dropped whole, and the page falls back
 * to its default pair.
 */
export const Route = createFileRoute("/_auth/compare")({
  validateSearch: (search: Record<string, unknown>): CompareSearch => {
    const parsed = CompareSearch.safeParse(search)
    return parsed.success ? parsed.data : {}
  },
})
