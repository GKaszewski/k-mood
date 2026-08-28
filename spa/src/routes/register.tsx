import { useState } from "react"
import { createFileRoute, Link } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { buttonLook } from "@/lib/button-style"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"
import { AuthCard } from "@/components/auth/auth-card"
import { FormError } from "@/components/auth/form-error"
import {
  PasswordStrength,
  isStrongEnough,
} from "@/components/auth/password-strength"
import { useRegister } from "@/hooks/use-auth"
import { useServerInfo } from "@/hooks/use-server"

export const Route = createFileRoute("/register")({ component: RegisterPage })

function RegisterPage() {
  const [username, setUsername] = useState("")
  const [email, setEmail] = useState("")
  const [password, setPassword] = useState("")
  const register = useRegister()
  const { data: serverInfo } = useServerInfo()

  if (serverInfo && !serverInfo.registrationOpen) {
    return <RegistrationClosed />
  }

  return (
    <AuthCard title="Create account" subtitle="Start tracking your mood">
      <form
        onSubmit={(event) => {
          event.preventDefault()
          register.mutate({ username, email, password })
        }}
        className="flex flex-col gap-4"
      >
        <div className="flex flex-col gap-2">
          <Label htmlFor="username">Username</Label>
          <Input
            id="username"
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            autoComplete="username"
            required
          />
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
            autoComplete="email"
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
            autoComplete="new-password"
            aria-describedby="password-strength"
            required
          />
          <PasswordStrength password={password} />
        </div>

        <FormError error={register.error} />

        <Button
          type="submit"
          disabled={register.isPending || !isStrongEnough(password)}
          className="w-full"
        >
          {register.isPending ? "Creating…" : "Create account"}
        </Button>

        <p className="text-center text-sm text-muted-foreground">
          Already have an account?{" "}
          <Link to="/login" className="text-primary underline">
            Sign in
          </Link>
        </p>
      </form>
    </AuthCard>
  )
}

function RegistrationClosed() {
  return (
    <main className="flex min-h-svh items-center justify-center p-4">
      <Card className="w-full max-w-sm">
        <CardContent>
          <Empty>
            <EmptyHeader>
              <EmptyTitle>Registration is closed</EmptyTitle>
              <EmptyDescription>
                This server is not taking new accounts. Ask whoever runs it for
                an invitation.
              </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
              <Link to="/login" className={buttonLook({ variant: "outline" })}>
                Back to sign in
              </Link>
            </EmptyContent>
          </Empty>
        </CardContent>
      </Card>
    </main>
  )
}
