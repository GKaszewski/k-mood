import { useEffect, useRef } from "react"

interface InfiniteScroll {
  hasMore: boolean
  isLoading: boolean
  loadMore: () => void
}

/**
 * Loads the next page when a sentinel element scrolls into view. Returns the
 * ref to hang on that sentinel.
 */
export function useInfiniteScroll({
  hasMore,
  isLoading,
  loadMore,
}: InfiniteScroll) {
  const sentinelRef = useRef<HTMLDivElement | null>(null)
  const loadMoreRef = useRef(loadMore)
  loadMoreRef.current = loadMore

  useEffect(() => {
    const sentinel = sentinelRef.current
    if (!sentinel || !hasMore || isLoading) return

    const observer = new IntersectionObserver((observed) => {
      if (observed.some((entry) => entry.isIntersecting)) {
        loadMoreRef.current()
      }
    })

    observer.observe(sentinel)
    return () => observer.disconnect()
  }, [hasMore, isLoading])

  return sentinelRef
}
