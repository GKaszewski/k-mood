import { useState } from "react"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { toast } from "@/components/ui/toast"
import { CollapsibleCard } from "@/components/settings/collapsible-card"
import {
  useConnectProvider,
  useDisconnectProvider,
  useProviderConnections,
} from "@/hooks/use-providers"

const PROVIDER = "subsonic"

export function SubsonicCard() {
  const connections = useProviderConnections()
  const connect = useConnectProvider(PROVIDER)
  const disconnect = useDisconnectProvider(PROVIDER)

  const [url, setUrl] = useState("")
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")

  const connected =
    connections.data?.some((c) => c.provider === PROVIDER) ?? false

  const submit = () => {
    connect.mutate(
      { url, username, password },
      {
        onSuccess: () => {
          setPassword("")
          toast.add({ title: "Subsonic connected", type: "success" })
        },
        onError: () => toast.add({ title: "Could not connect", type: "error" }),
      }
    )
  }

  return (
    <CollapsibleCard
      title="Music"
      description="Connect your own Subsonic server to fill in what you were listening to."
      summary={
        <Badge variant={connected ? "secondary" : "outline"}>
          {connected ? "connected" : "not connected"}
        </Badge>
      }
    >
      {connected ? (
        <FieldGroup>
          <Field orientation="horizontal">
            <FieldDescription>
              Subsonic is connected. Your password is stored encrypted and is
              never shown again.
            </FieldDescription>
            <Button
              variant="outline"
              onClick={() => disconnect.mutate()}
              disabled={disconnect.isPending}
            >
              Disconnect
            </Button>
          </Field>
        </FieldGroup>
      ) : (
        <FieldGroup>
          <Field>
            <FieldLabel htmlFor="subsonic-url">Address</FieldLabel>
            <Input
              id="subsonic-url"
              placeholder="https://music.example.com"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
            />
          </Field>
          <Field>
            <FieldLabel htmlFor="subsonic-username">Username</FieldLabel>
            <Input
              id="subsonic-username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              autoComplete="off"
            />
          </Field>
          <Field>
            <FieldLabel htmlFor="subsonic-password">Password</FieldLabel>
            <Input
              id="subsonic-password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoComplete="off"
            />
            <FieldDescription>
              Stored encrypted, and never shown again once saved.
            </FieldDescription>
          </Field>
          <Field orientation="horizontal">
            <Button
              onClick={submit}
              disabled={connect.isPending || !url || !username || !password}
            >
              {connect.isPending ? "Connecting…" : "Connect"}
            </Button>
          </Field>
        </FieldGroup>
      )}
    </CollapsibleCard>
  )
}
