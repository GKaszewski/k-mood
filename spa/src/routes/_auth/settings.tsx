import { useState } from "react"
import { createFileRoute, Link } from "@tanstack/react-router"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { SubsonicCard } from "@/components/settings/subsonic-card"
import { ApiTokensCard } from "@/components/settings/api-tokens-card"
import { RejectionsCard } from "@/components/settings/rejections-card"
import { CycleCard } from "@/components/settings/cycle-card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
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
import { useAuth, useLogout } from "@/hooks/use-auth"
import { usePushNotifications } from "@/hooks/use-push"
import { users, entries } from "@/api/client"
import { toast } from "@/components/ui/toast"
import {
  Bell,
  CalendarMinus,
  Clock,
  Download,
  ListChecks,
  LogOut,
  Pencil,
  Trash2,
  Upload,
  ChevronRight,
} from "lucide-react"

export const Route = createFileRoute("/_auth/settings")({
  component: SettingsPage,
})

function SettingsPage() {
  const { user } = useAuth()
  const logout = useLogout()
  const pushNotifications = usePushNotifications()
  const queryClient = useQueryClient()
  const [dialogOpen, setDialogOpen] = useState(false)
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)

  const clearData = useMutation({
    mutationFn: () => users.clearData(),
    onSuccess: () => {
      queryClient.invalidateQueries()
      setDialogOpen(false)
      toast.add({ title: "All data cleared", type: "success" })
    },
    onError: () => {
      toast.add({ title: "Failed to clear data", type: "error" })
    },
  })

  const deleteAccount = useMutation({
    mutationFn: () => users.deleteAccount(),
    onSuccess: () => {
      localStorage.removeItem("access_token")
      localStorage.removeItem("refresh_token")
      window.location.href = "/login"
    },
    onError: () => {
      toast.add({ title: "Failed to delete account", type: "error" })
    },
  })

  return (
    <div className="flex flex-col gap-6 py-6">
      <h1 className="text-xl font-bold">Settings</h1>

      <ProfileCard
        username={user?.username ?? ""}
        email={user?.email ?? ""}
        displayName={user?.displayName ?? null}
        timezone={user?.timezone ?? null}
      />

      <SubsonicCard />

      <CycleCard />

      <ApiTokensCard />

      <RejectionsCard />

      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Personalization</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-1 p-0">
          <Link to="/activities">
            <SettingsRow
              icon={<ListChecks className="h-4 w-4" />}
              label="Manage activities"
            />
          </Link>
          {pushNotifications.isSupported && pushNotifications.hasVapidKey && (
            <Link to="/reminders">
              <SettingsRow
                icon={<Clock className="h-4 w-4" />}
                label="Reminders"
              />
            </Link>
          )}
        </CardContent>
      </Card>

      {pushNotifications.isSupported && pushNotifications.hasVapidKey && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Notifications</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <Bell className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm">Push notifications</span>
              </div>
              <Button
                variant={pushNotifications.isSubscribed ? "outline" : "default"}
                size="sm"
                onClick={pushNotifications.toggle}
                disabled={pushNotifications.isLoading}
              >
                {pushNotifications.isLoading
                  ? "..."
                  : pushNotifications.isSubscribed
                    ? "Disable"
                    : "Enable"}
              </Button>
            </div>
            {pushNotifications.isSubscribed && (
              <Button
                variant="outline"
                size="sm"
                onClick={pushNotifications.sendTest}
                disabled={pushNotifications.isTesting}
              >
                {pushNotifications.isTesting
                  ? "Sending..."
                  : "Send test notification"}
              </Button>
            )}
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Data</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-1 p-0">
          <Link to="/export">
            <SettingsRow
              icon={<Download className="h-4 w-4" />}
              label="Export data"
            />
          </Link>
          <Link to="/import">
            <SettingsRow
              icon={<Upload className="h-4 w-4" />}
              label="Import data"
            />
          </Link>
        </CardContent>
      </Card>

      <ChangePasswordCard />

      <BulkDeleteCard />

      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Danger zone</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-2">
          <AlertDialog open={dialogOpen} onOpenChange={setDialogOpen}>
            <AlertDialogTrigger
              render={
                <Button
                  variant="outline"
                  className="w-full border-destructive/50 text-destructive"
                />
              }
            >
              <Trash2 className="mr-2 h-4 w-4" />
              Clear all data
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Clear all data?</AlertDialogTitle>
                <AlertDialogDescription>
                  This will permanently delete all your mood entries,
                  activities, and reminders. Your account will remain. This
                  cannot be undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={(e) => {
                    e.preventDefault()
                    clearData.mutate()
                  }}
                >
                  {clearData.isPending ? "Deleting..." : "Delete everything"}
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>

          <AlertDialog
            open={deleteDialogOpen}
            onOpenChange={setDeleteDialogOpen}
          >
            <AlertDialogTrigger
              render={<Button variant="destructive" className="w-full" />}
            >
              <Trash2 className="mr-2 h-4 w-4" />
              Delete account
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Delete your account?</AlertDialogTitle>
                <AlertDialogDescription>
                  This will permanently delete your account and all associated
                  data including entries, activities, photos, and voice memos.
                  This cannot be undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={(e) => {
                    e.preventDefault()
                    deleteAccount.mutate()
                  }}
                >
                  {deleteAccount.isPending
                    ? "Deleting..."
                    : "Delete my account"}
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </CardContent>
      </Card>

      <Button
        variant="destructive"
        onClick={() => logout.mutate()}
        disabled={logout.isPending}
        className="w-full"
      >
        <LogOut className="mr-2 h-4 w-4" />
        {logout.isPending ? "Signing out..." : "Sign out"}
      </Button>
    </div>
  )
}

function ProfileCard({
  username,
  email,
  displayName,
  timezone,
}: {
  username: string
  email: string
  displayName: string | null
  timezone: string | null
}) {
  const queryClient = useQueryClient()
  const [editing, setEditing] = useState(false)
  const [name, setName] = useState(displayName ?? "")
  const [tz, setTz] = useState(timezone ?? "")

  const updateProfile = useMutation({
    mutationFn: () =>
      users.updateProfile({
        displayName: name || null,
        timezone: tz || null,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["auth"] })
      setEditing(false)
      toast.add({ title: "Profile updated", type: "success" })
    },
  })

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-sm">Account</CardTitle>
        {!editing && (
          <Button
            variant="ghost"
            size="sm"
            onClick={() => {
              setName(displayName ?? "")
              setTz(timezone ?? "")
              setEditing(true)
            }}
          >
            <Pencil className="mr-1 h-3 w-3" />
            Edit
          </Button>
        )}
      </CardHeader>
      <CardContent className="flex flex-col gap-3 text-sm">
        <div className="flex justify-between">
          <span className="text-muted-foreground">Username</span>
          <span>{username}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-muted-foreground">Email</span>
          <span>{email}</span>
        </div>

        {editing ? (
          <>
            <div className="flex flex-col gap-1.5">
              <Label className="text-xs">Display name</Label>
              <Input
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Your name"
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label className="text-xs">Timezone</Label>
              <Input
                value={tz}
                onChange={(e) => setTz(e.target.value)}
                placeholder="e.g. Europe/Warsaw"
              />
              <p className="text-xs text-muted-foreground">
                Required for reminders to fire at the correct time
              </p>
            </div>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setEditing(false)}
                className="flex-1"
              >
                Cancel
              </Button>
              <Button
                size="sm"
                onClick={() => updateProfile.mutate()}
                disabled={updateProfile.isPending}
                className="flex-1"
              >
                {updateProfile.isPending ? "Saving..." : "Save"}
              </Button>
            </div>
          </>
        ) : (
          <>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Display name</span>
              <span>{displayName ?? "—"}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Timezone</span>
              <span>{timezone ?? "—"}</span>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  )
}

function ChangePasswordCard() {
  const [open, setOpen] = useState(false)
  const [currentPassword, setCurrentPassword] = useState("")
  const [newPassword, setNewPassword] = useState("")

  const changePassword = useMutation({
    mutationFn: () => users.changePassword({ currentPassword, newPassword }),
    onSuccess: () => {
      setOpen(false)
      setCurrentPassword("")
      setNewPassword("")
      toast.add({ title: "Password changed", type: "success" })
    },
  })

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-sm">Security</CardTitle>
        {!open && (
          <Button variant="ghost" size="sm" onClick={() => setOpen(true)}>
            Change password
          </Button>
        )}
      </CardHeader>
      {open && (
        <CardContent className="flex flex-col gap-3">
          <div className="flex flex-col gap-1.5">
            <Label className="text-xs">Current password</Label>
            <Input
              type="password"
              value={currentPassword}
              onChange={(e) => setCurrentPassword(e.target.value)}
            />
          </div>
          <div className="flex flex-col gap-1.5">
            <Label className="text-xs">New password</Label>
            <Input
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
            />
          </div>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                setOpen(false)
                setCurrentPassword("")
                setNewPassword("")
              }}
              className="flex-1"
            >
              Cancel
            </Button>
            <Button
              size="sm"
              onClick={() => changePassword.mutate()}
              disabled={
                changePassword.isPending || !currentPassword || !newPassword
              }
              className="flex-1"
            >
              {changePassword.isPending ? "Changing..." : "Change password"}
            </Button>
          </div>
        </CardContent>
      )}
    </Card>
  )
}

function BulkDeleteCard() {
  const queryClient = useQueryClient()
  const [open, setOpen] = useState(false)
  const [from, setFrom] = useState("")
  const [to, setTo] = useState("")

  const bulkDelete = useMutation({
    mutationFn: () => entries.deleteByDateRange({ from, to }),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: ["entries"] })
      setOpen(false)
      setFrom("")
      setTo("")
      toast.add({
        title: `${data.affectedCount} entries deleted`,
        type: "success",
      })
    },
  })

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-sm">Bulk actions</CardTitle>
        {!open && (
          <Button variant="ghost" size="sm" onClick={() => setOpen(true)}>
            <CalendarMinus className="mr-1 h-3 w-3" />
            Delete by date
          </Button>
        )}
      </CardHeader>
      {open && (
        <CardContent className="flex flex-col gap-3">
          <p className="text-xs text-muted-foreground">
            Delete all entries within a date range. This cannot be undone.
          </p>
          <div className="flex gap-2">
            <div className="flex flex-1 flex-col gap-1.5">
              <Label className="text-xs">From</Label>
              <Input
                type="date"
                value={from}
                onChange={(e) => setFrom(e.target.value)}
              />
            </div>
            <div className="flex flex-1 flex-col gap-1.5">
              <Label className="text-xs">To</Label>
              <Input
                type="date"
                value={to}
                onChange={(e) => setTo(e.target.value)}
              />
            </div>
          </div>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                setOpen(false)
                setFrom("")
                setTo("")
              }}
              className="flex-1"
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={() => bulkDelete.mutate()}
              disabled={!from || !to || bulkDelete.isPending}
              className="flex-1"
            >
              {bulkDelete.isPending ? "Deleting..." : "Delete entries"}
            </Button>
          </div>
        </CardContent>
      )}
    </Card>
  )
}

function SettingsRow({
  icon,
  label,
}: {
  icon: React.ReactNode
  label: string
}) {
  return (
    <div className="flex items-center justify-between px-6 py-3 transition-colors hover:bg-muted/50">
      <div className="flex items-center gap-3">
        <span className="text-muted-foreground">{icon}</span>
        <span className="text-sm">{label}</span>
      </div>
      <ChevronRight className="h-4 w-4 text-muted-foreground" />
    </div>
  )
}
