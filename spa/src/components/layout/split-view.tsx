import type { ReactNode } from "react"

interface SplitViewProps {
  /** What the reader sets and the headline figures: kept in view on a wide screen. */
  panel: ReactNode
  children: ReactNode
}

/**
 * A control panel and the results it governs. On a phone the panel simply comes
 * first; on a wide screen it stays pinned at the side, so changing a setting
 * halfway down the results never means scrolling back to the top.
 */
export function SplitView({ panel, children }: SplitViewProps) {
  return (
    <div className="flex flex-col gap-4 py-6 lg:grid lg:grid-cols-[22rem_minmax(0,1fr)] lg:items-start lg:gap-6">
      <aside className="flex flex-col gap-4 lg:sticky lg:top-6">{panel}</aside>
      <div className="flex min-w-0 flex-col gap-4">{children}</div>
    </div>
  )
}
