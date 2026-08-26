import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { toast } from "@/components/ui/toast"
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
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Music</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        {connected ? (
          <div className="flex items-center justify-between gap-3">
            <p className="text-sm text-muted-foreground">
              Subsonic is connected. Your password is stored encrypted and is
              never shown again.
            </p>
            <Button
              variant="outline"
              size="sm"
              onClick={() => disconnect.mutate()}
              disabled={disconnect.isPending}
            >
              Disconnect
            </Button>
          </div>
        ) : (
          <>
            <p className="text-xs text-muted-foreground">
              Connect your own Subsonic server to fill in what you were
              listening to.
            </p>
            <Input
              placeholder="https://music.example.com"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
            />
            <Input
              placeholder="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
            <Input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <Button
              size="sm"
              onClick={submit}
              disabled={connect.isPending || !url || !username || !password}
            >
              {connect.isPending ? "Connecting…" : "Connect"}
            </Button>
          </>
        )}
      </CardContent>
    </Card>
  )
}
