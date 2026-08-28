import { useMoodStats } from "./use-stats"

/** The first and last day this account ever logged, if it has logged at all. */
export function useDateBounds() {
  const { data: stats } = useMoodStats("all")

  if (!stats?.firstLoggedOn || !stats.lastLoggedOn) return { data: null }

  return {
    data: {
      oldest: new Date(stats.firstLoggedOn),
      newest: new Date(stats.lastLoggedOn),
    },
  }
}
