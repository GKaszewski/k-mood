import { useState } from "react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import {
  forgetServer,
  isServerChosen,
  looksLikeAServer,
  reachable,
  rememberServer,
  serverUrl,
} from "@/api/server-url"

type Checking = "idle" | "asking" | "unreachable"

export function ServerPicker({ version }: { version?: string }) {
  const current = serverUrl()
  const chosen = isServerChosen()

  const [typed, setTyped] = useState(current)
  const [checking, setChecking] = useState<Checking>("idle")

  const tidy = typed.trim()
  const usable = looksLikeAServer(tidy) && tidy !== current

  const connect = async () => {
    setChecking("asking")

    if (await reachable(tidy)) {
      rememberServer(tidy)
      return
    }

    setChecking("unreachable")
  }

  return (
    <FieldGroup>
      <Field orientation="horizontal">
        <FieldLabel>Connected to</FieldLabel>
        <Badge variant="secondary">{current || "this site"}</Badge>
        {version && <Badge variant="outline">v{version}</Badge>}
      </Field>

      <Field>
        <FieldLabel htmlFor="server-url">Address</FieldLabel>
        <Input
          id="server-url"
          placeholder="https://k-mood.example.com"
          value={typed}
          aria-invalid={tidy.length > 0 && !looksLikeAServer(tidy)}
          onChange={(event) => {
            setTyped(event.target.value)
            setChecking("idle")
          }}
        />
        <FieldDescription>
          The server must allow this site as a CORS origin, which whoever runs
          it sets in its configuration.
        </FieldDescription>
      </Field>

      {checking === "unreachable" && (
        <Alert variant="destructive">
          <AlertTitle>No k-mood server answered</AlertTitle>
          <AlertDescription>
            Nothing responded at that address, or it refused this site as an
            origin. The address was not saved.
          </AlertDescription>
        </Alert>
      )}

      <Field orientation="horizontal">
        <Button disabled={!usable || checking === "asking"} onClick={connect}>
          {checking === "asking" && <Spinner data-icon="inline-start" />}
          Connect
        </Button>
        {chosen && (
          <Button variant="outline" onClick={forgetServer}>
            Use this site again
          </Button>
        )}
      </Field>
    </FieldGroup>
  )
}
