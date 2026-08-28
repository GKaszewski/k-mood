import { useSyncExternalStore } from "react"

function subscribe(onChange: () => void): () => void {
  window.addEventListener("online", onChange)
  window.addEventListener("offline", onChange)

  return () => {
    window.removeEventListener("online", onChange)
    window.removeEventListener("offline", onChange)
  }
}

/**
 * Whether the browser believes it has a connection. It can be wrong — a
 * captive portal reads as online — so this steers the interface, and the
 * outbox still decides what actually sent.
 */
export function useOnline(): boolean {
  return useSyncExternalStore(
    subscribe,
    () => navigator.onLine,
    () => true
  )
}
