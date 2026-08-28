const SECONDS_PER_MINUTE = 60

/** Whole seconds as `m:ss`. */
export function formatDuration(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) return "0:00"

  const minutes = Math.floor(seconds / SECONDS_PER_MINUTE)
  const remainder = Math.floor(seconds % SECONDS_PER_MINUTE)

  return `${minutes}:${remainder.toString().padStart(2, "0")}`
}
