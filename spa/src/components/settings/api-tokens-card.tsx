import { useState } from "react"
import { formatDistanceToNow } from "date-fns"
import { Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { toast } from "@/components/ui/toast"
import {
  useApiTokens,
  useMintApiToken,
  useRevokeApiToken,
} from "@/hooks/use-api-tokens"
import type { ApiTokenResponse } from "@/api/schema"

const NAME_RULE = /^[a-z0-9-]+$/

export function ApiTokensCard() {
  const { data: existing } = useApiTokens()
  const mint = useMintApiToken()
  const revoke = useRevokeApiToken()

  const [name, setName] = useState("")
  const [justMinted, setJustMinted] = useState<string | null>(null)

  const normalised = name.trim().toLowerCase()
  const canMint = normalised.length > 0 && NAME_RULE.test(normalised)

  const submit = () => {
    mint.mutate(normalised, {
      onSuccess: (minted) => {
        setJustMinted(minted.secret)
        setName("")
      },
      onError: (error) =>
        toast.add({
          title: error instanceof Error ? error.message : "Could not mint",
          type: "error",
        }),
    })
  }

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Importer tokens</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <p className="text-xs text-muted-foreground">
          For automations that send daily metrics — a Shortcut, Tasker, a cron
          script. A token can only write metrics: it cannot read your entries or
          touch your account. Tokens do not expire, so revoke any you stop
          using.
        </p>

        {justMinted && (
          <div className="flex flex-col gap-1.5 rounded-md border border-dashed p-3">
            <Label className="text-xs">
              Copy this now — it is not shown again
            </Label>
            <code className="text-xs break-all">{justMinted}</code>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setJustMinted(null)}
            >
              Done
            </Button>
          </div>
        )}

        <div className="flex flex-col gap-1.5">
          <Label className="text-xs">
            Name — becomes the source recorded on its data
          </Label>
          <div className="flex gap-2">
            <Input
              placeholder="iphone-shortcuts"
              value={name}
              onChange={(event) => setName(event.target.value)}
            />
            <Button
              size="sm"
              disabled={!canMint || mint.isPending}
              onClick={submit}
            >
              {mint.isPending ? "Minting…" : "Mint"}
            </Button>
          </div>
          {normalised.length > 0 && !canMint && (
            <span className="text-xs text-destructive">
              Lowercase letters, digits and hyphens only.
            </span>
          )}
        </div>

        {existing?.length ? (
          <div className="flex flex-col gap-2">
            {existing.map((token) => (
              <TokenRow
                key={token.id}
                token={token}
                onRevoke={() => revoke.mutate(token.id)}
              />
            ))}
          </div>
        ) : (
          <span className="text-xs text-muted-foreground">No tokens yet.</span>
        )}
      </CardContent>
    </Card>
  )
}

function TokenRow({
  token,
  onRevoke,
}: {
  token: ApiTokenResponse
  onRevoke: () => void
}) {
  const lastUsed = token.lastUsedAt
    ? `used ${formatDistanceToNow(new Date(token.lastUsedAt), { addSuffix: true })}`
    : "never used"

  return (
    <div className="flex items-center justify-between gap-2">
      <div className="flex min-w-0 flex-col">
        <span className="truncate text-xs font-medium">{token.name}</span>
        <span className="text-[10px] text-muted-foreground">{lastUsed}</span>
      </div>
      <Button variant="ghost" size="icon" onClick={onRevoke}>
        <Trash2 className="h-4 w-4 text-destructive" />
      </Button>
    </div>
  )
}
