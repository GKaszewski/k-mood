import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { groupNamesById } from "@/lib/activity-groups"
import type { ActivityResponse } from "@/api/schema"

interface EntryActivitiesProps {
  activityIds: string[]
  catalogue: ActivityResponse[]
}

export function EntryActivities({
  activityIds,
  catalogue,
}: EntryActivitiesProps) {
  if (!activityIds.length) return null

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Activities</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {groupNamesById(activityIds, catalogue).map(({ category, items }) => (
          <div key={category}>
            <p className="mb-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
              {category}
            </p>
            <div className="flex flex-wrap gap-2">
              {items.map((activity) => (
                <Badge key={activity.id} variant="outline" className="text-xs">
                  {activity.name}
                </Badge>
              ))}
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  )
}
