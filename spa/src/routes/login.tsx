import { useState } from "react"
import { createFileRoute, Link } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { AuthCard } from "@/components/auth/auth-card"
import { FormError } from "@/components/auth/form-error"
import { ServerPicker } from "@/components/settings/server-picker"
import { useLogin } from "@/hooks/use-auth"
import { useServerInfo } from "@/hooks/use-server"

export const Route = createFileRoute("/login")({ component: LoginPage })

function LoginPage() {
  const [identifier, setIdentifier] = useState("")
  const [password, setPassword] = useState("")
  const login = useLogin()
  const { data: serverInfo } = useServerInfo()

  return (
    <AuthCard title="K-Mood" subtitle="Track your mood, understand yourself">
      <form
        onSubmit={(event) => {
          event.preventDefault()
          login.mutate({ identifier, password })
        }}
        className="flex flex-col gap-4"
      >
        <div className="flex flex-col gap-2">
          <Label htmlFor="identifier">Username or email</Label>
          <Input
            id="identifier"
            value={identifier}
            onChange={(event) => setIdentifier(event.target.value)}
            autoComplete="username"
            required
          />
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            autoComplete="current-password"
            required
          />
        </div>

        <FormError error={login.error} />

        <Button type="submit" disabled={login.isPending} className="w-full">
          {login.isPending ? "Signing in…" : "Sign in"}
        </Button>

        <Dialog>
          <DialogTrigger
            render={
              <Button
                variant="link"
                size="sm"
                type="button"
                className="w-full"
              />
            }
          >
            Connect to a different server
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Choose a server</DialogTitle>
              <DialogDescription>
                This app is not tied to one k-mood server. Point it at whichever
                one holds your journal.
              </DialogDescription>
            </DialogHeader>
            <ServerPicker version={serverInfo?.version} />
          </DialogContent>
        </Dialog>

        {serverInfo?.registrationOpen && (
          <p className="text-center text-sm text-muted-foreground">
            Don&apos;t have an account?{" "}
            <Link to="/register" className="text-primary underline">
              Register
            </Link>
          </p>
        )}
      </form>
    </AuthCard>
  )
}
