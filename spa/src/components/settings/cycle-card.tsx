import { useState } from "react"
import { format } from "date-fns"
import { Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import {
  useCycle,
  useForgetCycleStart,
  usePreferences,
  useRecordCycleStart,
  useSetCycleTracking,
} from "@/hooks/use-cycle"

export function CycleCard() {
  const { data: preferences } = usePreferences()
  const setTracking = useSetCycleTracking()

  const tracking = preferences?.tracksCycle ?? false
  const { data: view } = useCycle(tracking)

  const today = format(new Date(), "yyyy-MM-dd")
  const [start, setStart] = useState(today)
  const record = useRecordCycleStart()
  const forget = useForgetCycleStart()

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between pb-2">
        <CardTitle className="text-sm">Cycle</CardTitle>
        <Switch
          checked={tracking}
          onCheckedChange={(next) => setTracking.mutate(next)}
        />
      </CardHeader>

      {tracking && (
        <CardContent className="flex flex-col gap-3">
          <p className="text-xs text-muted-foreground">
            Record the day a period begins — once a cycle, not once a day. The
            cycle day for every other date is worked out from it, so correcting
            a start fixes everything that followed.
          </p>

          {view?.today ? (
            <p className="text-xs">
              Today is <span className="font-medium">day {view.today.day}</span>{" "}
              of a cycle that usually runs {view.usualLength} days.
            </p>
          ) : (
            <p className="text-xs text-muted-foreground">
              No cycle day for today yet — record a start below.
            </p>
          )}

          <div className="flex flex-col gap-1.5">
            <Label className="text-xs">A period began on</Label>
            <div className="flex gap-2">
              <Input
                type="date"
                value={start}
                max={today}
                onChange={(event) => setStart(event.target.value)}
              />
              <Button
                size="sm"
                disabled={record.isPending}
                onClick={() => record.mutate(start)}
              >
                Record
              </Button>
            </div>
          </div>

          {view?.starts.length ? (
            <div className="flex flex-col gap-1">
              {[...view.starts].reverse().map((day) => (
                <div key={day} className="flex items-center justify-between">
                  <span className="text-xs">{day}</span>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => forget.mutate(day)}
                  >
                    <Trash2 className="h-3.5 w-3.5 text-destructive" />
                  </Button>
                </div>
              ))}
            </div>
          ) : (
            <span className="text-xs text-muted-foreground">
              Nothing recorded yet.
            </span>
          )}
        </CardContent>
      )}
    </Card>
  )
}
