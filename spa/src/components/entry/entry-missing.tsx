import { Link } from "@tanstack/react-router"
import { buttonLook } from "@/lib/button-style"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"

export function EntryMissing() {
  return (
    <Empty className="py-12">
      <EmptyHeader>
        <EmptyTitle>Entry not found</EmptyTitle>
        <EmptyDescription>
          It may have been deleted, or the link may be wrong.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Link to="/diary" className={buttonLook({ variant: "outline" })}>
          Back to the diary
        </Link>
      </EmptyContent>
    </Empty>
  )
}
