import { useState } from "react"
import { createFileRoute } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Switch } from "@/components/ui/switch"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import {
  useReminders,
  useCreateReminder,
  useUpdateReminder,
  useDeleteReminder,
} from "@/hooks/use-reminders"
import type { ReminderResponse, CreateReminderRequest } from "@/api/schema"
import { Plus, Trash2, ArrowLeft } from "lucide-react"
import { Link } from "@tanstack/react-router"

export const Route = createFileRoute("/_auth/reminders")({
  component: RemindersPage,
})

const DAYS = [
  { key: "monday", label: "Mon" },
  { key: "tuesday", label: "Tue" },
  { key: "wednesday", label: "Wed" },
  { key: "thursday", label: "Thu" },
  { key: "friday", label: "Fri" },
  { key: "saturday", label: "Sat" },
  { key: "sunday", label: "Sun" },
] as const

type DayKey = (typeof DAYS)[number]["key"]

function RemindersPage() {
  const { data: reminders, isLoading } = useReminders()
  const createReminder = useCreateReminder()
  const [showForm, setShowForm] = useState(false)

  const handleCreate = (schedule: CreateReminderRequest) => {
    createReminder.mutate(schedule, {
      onSuccess: () => setShowForm(false),
    })
  }

  return (
    <div className="flex flex-col gap-6 py-6">
      <div className="flex items-center gap-3">
        <Link to="/settings">
          <Button variant="ghost" size="icon">
            <ArrowLeft className="h-5 w-5" />
          </Button>
        </Link>
        <h1 className="text-xl font-bold">Reminders</h1>
      </div>

      {isLoading && <p className="text-sm text-muted-foreground">Loading...</p>}

      {reminders?.map((reminder) => (
        <ReminderCard key={reminder.id} reminder={reminder} />
      ))}

      {reminders?.length === 0 && !showForm && (
        <p className="text-center text-sm text-muted-foreground">
          No reminders yet. Create one to get notified.
        </p>
      )}

      {showForm ? (
        <ReminderForm
          onSave={handleCreate}
          onCancel={() => setShowForm(false)}
          isPending={createReminder.isPending}
        />
      ) : (
        <Button onClick={() => setShowForm(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Add reminder
        </Button>
      )}
    </div>
  )
}

function ReminderCard({ reminder }: { reminder: ReminderResponse }) {
  const updateReminder = useUpdateReminder()
  const deleteReminder = useDeleteReminder()
  const [editing, setEditing] = useState(false)

  const handleToggle = (enabled: boolean) => {
    updateReminder.mutate({ id: reminder.id, body: { enabled } })
  }

  const handleUpdate = (schedule: CreateReminderRequest) => {
    updateReminder.mutate(
      { id: reminder.id, body: { schedule } },
      { onSuccess: () => setEditing(false) },
    )
  }

  if (editing) {
    return (
      <ReminderForm
        initial={reminder.schedule}
        onSave={handleUpdate}
        onCancel={() => setEditing(false)}
        isPending={updateReminder.isPending}
      />
    )
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between pb-2">
        <CardTitle className="text-sm">
          {reminder.enabled ? "Active" : "Disabled"}
        </CardTitle>
        <Switch
          checked={reminder.enabled}
          onCheckedChange={handleToggle}
          disabled={updateReminder.isPending}
        />
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <div className="flex flex-wrap gap-2">
          {DAYS.map(({ key, label }) => {
            const time = reminder.schedule[key]
            return (
              <DayPill key={key} label={label} time={time} />
            )
          })}
        </div>

        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setEditing(true)}
            className="flex-1"
          >
            Edit
          </Button>
          <AlertDialog>
            <AlertDialogTrigger
              render={
                <Button variant="outline" size="sm" className="text-destructive" />
              }
            >
              <Trash2 className="h-4 w-4" />
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Delete reminder?</AlertDialogTitle>
                <AlertDialogDescription>
                  This reminder will be permanently removed.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={(e) => {
                    e.preventDefault()
                    deleteReminder.mutate(reminder.id)
                  }}
                >
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </CardContent>
    </Card>
  )
}

function DayPill({ label, time }: { label: string; time: string | null }) {
  return (
    <div
      className={`flex flex-col items-center rounded-lg border px-3 py-2 text-xs ${
        time
          ? "border-primary/30 bg-primary/10 text-foreground"
          : "border-transparent bg-muted/50 text-muted-foreground"
      }`}
    >
      <span className="font-medium">{label}</span>
      <span>{time ? formatTime(time) : "—"}</span>
    </div>
  )
}

function formatTime(time: string): string {
  const [h, m] = time.split(":")
  const hour = parseInt(h)
  const ampm = hour >= 12 ? "PM" : "AM"
  const h12 = hour % 12 || 12
  return `${h12}:${m} ${ampm}`
}

function ReminderForm({
  initial,
  onSave,
  onCancel,
  isPending,
}: {
  initial?: Record<DayKey, string | null>
  onSave: (schedule: CreateReminderRequest) => void
  onCancel: () => void
  isPending: boolean
}) {
  const defaultTime = "20:00"
  const [schedule, setSchedule] = useState<Record<DayKey, string | null>>(() => {
    if (initial) return { ...initial }
    return {
      monday: null,
      tuesday: null,
      wednesday: null,
      thursday: null,
      friday: null,
      saturday: null,
      sunday: null,
    }
  })

  const toggleDay = (day: DayKey) => {
    setSchedule((prev) => ({
      ...prev,
      [day]: prev[day] ? null : defaultTime,
    }))
  }

  const setTime = (day: DayKey, time: string) => {
    setSchedule((prev) => ({ ...prev, [day]: time }))
  }

  const selectAll = () => {
    const allSet = DAYS.every(({ key }) => schedule[key])
    const newSchedule = { ...schedule }
    for (const { key } of DAYS) {
      newSchedule[key] = allSet ? null : defaultTime
    }
    setSchedule(newSchedule)
  }

  const hasAnyDay = DAYS.some(({ key }) => schedule[key])

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">
          {initial ? "Edit reminder" : "New reminder"}
        </CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex items-center justify-between">
          <span className="text-xs text-muted-foreground">Select days</span>
          <Button variant="ghost" size="sm" onClick={selectAll} className="text-xs">
            {DAYS.every(({ key }) => schedule[key]) ? "Clear all" : "Every day"}
          </Button>
        </div>

        <div className="flex flex-col gap-2">
          {DAYS.map(({ key, label }) => (
            <div key={key} className="flex items-center justify-between gap-3">
              <button
                type="button"
                onClick={() => toggleDay(key)}
                className={`w-12 rounded-md py-1.5 text-center text-xs font-medium transition-colors ${
                  schedule[key]
                    ? "bg-primary text-primary-foreground"
                    : "bg-muted text-muted-foreground"
                }`}
              >
                {label}
              </button>
              {schedule[key] && (
                <input
                  type="time"
                  value={schedule[key]!}
                  onChange={(e) => setTime(key, e.target.value)}
                  className="h-9 rounded-md border bg-transparent px-3 text-sm"
                />
              )}
              {!schedule[key] && (
                <span className="text-xs text-muted-foreground">Off</span>
              )}
            </div>
          ))}
        </div>

        <div className="flex gap-2">
          <Button variant="outline" onClick={onCancel} className="flex-1">
            Cancel
          </Button>
          <Button
            onClick={() => onSave(schedule)}
            disabled={!hasAnyDay || isPending}
            className="flex-1"
          >
            {isPending ? "Saving..." : "Save"}
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}
