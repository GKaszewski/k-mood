/**
 * Registration, and noticing when a new build is waiting to take over. The
 * worker never activates itself, so this is the only path to an update.
 */

const SKIP_WAITING = "SKIP_WAITING"

export type UpdateReady = (applyUpdate: () => void) => void

function whenWaiting(
  registration: ServiceWorkerRegistration,
  onReady: UpdateReady
): void {
  const waiting = registration.waiting
  if (!waiting) return

  onReady(() => {
    waiting.postMessage({ type: SKIP_WAITING })
  })
}

export function registerServiceWorker(onUpdateReady: UpdateReady): void {
  if (!("serviceWorker" in navigator)) return

  // Never in development. The worker serves same-origin assets cache-first,
  // which hands the dev server's own modules back stale and breaks hot
  // reloading in ways that look like impossible application bugs.
  if (import.meta.env.DEV) return

  navigator.serviceWorker.register("/sw.js").then(
    (registration) => {
      // A build that finished installing before this page loaded.
      whenWaiting(registration, onUpdateReady)

      registration.addEventListener("updatefound", () => {
        const installing = registration.installing
        if (!installing) return

        installing.addEventListener("statechange", () => {
          // Only an update: with no controller this is the very first install.
          if (
            installing.state === "installed" &&
            navigator.serviceWorker.controller
          ) {
            whenWaiting(registration, onUpdateReady)
          }
        })
      })
    },
    () => {
      // A refused registration leaves the app working, just without offline assets.
    }
  )

  let reloading = false
  navigator.serviceWorker.addEventListener("controllerchange", () => {
    if (reloading) return
    reloading = true
    window.location.reload()
  })
}

const RELOADED_FOR = "reloadedForMissingChunk"

/**
 * A deploy replaces every hashed chunk. A tab that was open across it still
 * holds the old index, so the first lazy route it opens asks for a file that
 * is no longer there and the screen dead-ends. One reload picks up the new
 * index; the marker makes sure a genuinely missing chunk cannot loop.
 */
export function isMissingChunk(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error)

  return (
    message.includes("Failed to fetch dynamically imported module") ||
    message.includes("error loading dynamically imported module") ||
    message.includes("Importing a module script failed")
  )
}

export function reloadForMissingChunk(): boolean {
  try {
    if (sessionStorage.getItem(RELOADED_FOR)) return false
    sessionStorage.setItem(RELOADED_FOR, "1")
  } catch {
    return false
  }

  window.location.reload()
  return true
}

export function forgetMissingChunkReload(): void {
  try {
    sessionStorage.removeItem(RELOADED_FOR)
  } catch {
    // Nothing to forget.
  }
}
