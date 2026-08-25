import { useState } from "react"
import { Archive, ArchiveRestore, Merge, Pencil, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { cn } from "@/lib/utils"
import {
  useRenameActivity,
  useArchiveActivity,
  useUnarchiveActivity,
  useDeleteActivity,
  useReplaceActivity,
} from "@/hooks/use-manage-activities"
import { useActivities } from "@/hooks/use-activities"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog"

interface ActivityRowProps {
  activity: {
    id: string
    name: string
    archived: boolean
  }
}

export function ActivityRow({ activity }: ActivityRowProps) {
  const [editing, setEditing] = useState(false)
  const [editName, setEditName] = useState(activity.name)
  const [merging, setMerging] = useState(false)
  const [mergeTarget, setMergeTarget] = useState<string | null>(null)
  const rename = useRenameActivity()
  const archive = useArchiveActivity()
  const unarchive = useUnarchiveActivity()
  const del = useDeleteActivity()
  const replace = useReplaceActivity()
  const { data: allActivities } = useActivities()

  const handleRename = () => {
    if (!editName.trim() || editName === activity.name) {
      setEditing(false)
      return
    }
    rename.mutate(
      { id: activity.id, newName: editName.trim() },
      { onSuccess: () => setEditing(false) }
    )
  }

  const otherActivities = (allActivities ?? []).filter(
    (a) => a.id !== activity.id && !a.archived
  )

  const handleMergeConfirm = () => {
    if (!mergeTarget) return
    replace.mutate(
      { oldActivityId: activity.id, newActivityId: mergeTarget },
      {
        onSuccess: () => {
          setMerging(false)
          setMergeTarget(null)
        },
      }
    )
  }

  return (
    <>
      <div
        className={cn(
          "flex items-center justify-between px-4 py-2",
          activity.archived && "opacity-50"
        )}
      >
        {editing ? (
          <Input
            value={editName}
            onChange={(e) => setEditName(e.target.value)}
            onBlur={handleRename}
            onKeyDown={(e) => e.key === "Enter" && handleRename()}
            className="h-7 text-sm"
            autoFocus
          />
        ) : (
          <span className="text-sm">{activity.name}</span>
        )}

        <div className="flex items-center gap-1">
          {!activity.archived && (
            <>
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => {
                  setEditName(activity.name)
                  setEditing(true)
                }}
              >
                <Pencil className="h-3 w-3" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => setMerging(true)}
              >
                <Merge className="h-3 w-3" />
              </Button>
            </>
          )}

          {activity.archived ? (
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={() => unarchive.mutate(activity.id)}
            >
              <ArchiveRestore className="h-3 w-3" />
            </Button>
          ) : (
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={() => archive.mutate(activity.id)}
            >
              <Archive className="h-3 w-3" />
            </Button>
          )}

          <Button
            variant="ghost"
            size="icon"
            className="h-7 w-7 text-destructive"
            onClick={() => del.mutate(activity.id)}
          >
            <Trash2 className="h-3 w-3" />
          </Button>
        </div>
      </div>

      <AlertDialog open={merging} onOpenChange={setMerging}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>
              Merge "{activity.name}" into another activity
            </AlertDialogTitle>
            <AlertDialogDescription>
              All entries with "{activity.name}" will be updated to use the
              selected activity instead.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <select
            value={mergeTarget ?? ""}
            onChange={(e) => setMergeTarget(e.target.value || null)}
            className="h-9 w-full rounded-md border bg-transparent px-3 text-sm"
          >
            <option value="">Select target activity</option>
            {otherActivities.map((a) => (
              <option key={a.id} value={a.id}>
                {a.name}
              </option>
            ))}
          </select>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              disabled={!mergeTarget || replace.isPending}
              onClick={(e) => {
                e.preventDefault()
                handleMergeConfirm()
              }}
            >
              {replace.isPending ? "Merging..." : "Merge"}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}
