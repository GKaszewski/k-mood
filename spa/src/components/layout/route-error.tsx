import { useEffect } from "react"
import { useRouter } from "@tanstack/react-router"
import { RotateCw } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"
import {
  forgetMissingChunkReload,
  isMissingChunk,
  reloadForMissingChunk,
} from "@/lib/service-worker"

/**
 * What a screen shows when it fails, instead of taking the whole app down with
 * it. A chunk that went missing under a deploy heals itself with one reload;
 * anything else is offered a retry.
 */
export function RouteError({ error }: { error: unknown }) {
  const router = useRouter()
  const staleBuild = isMissingChunk(error)

  useEffect(() => {
    if (staleBuild) reloadForMissingChunk()
    else forgetMissingChunkReload()
  }, [staleBuild])

  if (staleBuild) {
    return (
      <Empty className="py-16">
        <EmptyHeader>
          <EmptyTitle>Updating…</EmptyTitle>
          <EmptyDescription>
            A newer version is available. Picking it up now.
          </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button variant="outline" onClick={() => window.location.reload()}>
            <RotateCw className="mr-2 h-4 w-4" aria-hidden="true" />
            Reload
          </Button>
        </EmptyContent>
      </Empty>
    )
  }

  return (
    <Empty className="py-16">
      <EmptyHeader>
        <EmptyTitle>This screen did not load</EmptyTitle>
        <EmptyDescription>
          {error instanceof Error ? error.message : "Something went wrong."}
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Button variant="outline" onClick={() => router.invalidate()}>
          <RotateCw className="mr-2 h-4 w-4" aria-hidden="true" />
          Try again
        </Button>
      </EmptyContent>
    </Empty>
  )
}
