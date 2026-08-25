import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ActivityRow } from "./activity-row"

interface ActivityGroupProps {
  category: string
  activities: Array<{
    id: string
    name: string
    category?: string | null
    archived: boolean
  }>
}

export function ActivityGroup({ category, activities }: ActivityGroupProps) {
  const active = activities.filter((a) => !a.archived)
  const archived = activities.filter((a) => a.archived)

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">{category}</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-1 p-0">
        {active.map((a) => (
          <ActivityRow key={a.id} activity={a} />
        ))}
        {archived.length > 0 && (
          <div className="border-t px-4 py-2">
            <p className="mb-1 text-xs text-muted-foreground">Archived</p>
            {archived.map((a) => (
              <ActivityRow key={a.id} activity={a} />
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
