import { formatDistanceToNow } from "date-fns"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { useRejections } from "@/hooks/use-rejections"
import type { RejectionResponse } from "@/api/schema"

export function RejectionsCard() {
  const { data: rejections, isLoading } = useRejections()

  if (isLoading) return <Skeleton className="h-24" />
  if (!rejections?.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">
          Readings that could not be used
        </CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <p className="text-xs text-muted-foreground">
          Nothing here was stored. An automation cannot show you an error, so
          whatever it sent that could not be read is listed here instead. Only
          the most recent are kept.
        </p>

        {rejections.map((rejection) => (
          <RejectionRow key={rejection.id} rejection={rejection} />
        ))}
      </CardContent>
    </Card>
  )
}

function RejectionRow({ rejection }: { rejection: RejectionResponse }) {
  const source =
    rejection.origin === "storedRow"
      ? "stored by an older build"
      : (rejection.provider ?? "an importer")

  return (
    <div className="flex flex-col gap-0.5 border-l-2 border-destructive/40 pl-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className="text-xs font-medium">
          {rejection.kind}
          {rejection.value !== null && (
            <span className="font-normal text-muted-foreground">
              {" "}
              = {rejection.value}
            </span>
          )}
        </span>
        <span className="shrink-0 text-[10px] text-muted-foreground">
          {rejection.date ?? "no date"}
        </span>
      </div>
      <span className="text-[10px] text-muted-foreground">
        {rejection.reason}
      </span>
      <span className="text-[10px] text-muted-foreground">
        from {source},{" "}
        {formatDistanceToNow(new Date(rejection.recordedAt), {
          addSuffix: true,
        })}
      </span>
    </div>
  )
}
