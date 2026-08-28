import { useState } from "react"
import { formatDistanceToNow } from "date-fns"
import { Trash2 } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { CollapsibleCard } from "@/components/settings/collapsible-card"
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { toast } from "@/components/ui/toast"
import {
  useApiTokens,
  useMintApiToken,
  useRevokeApiToken,
} from "@/hooks/use-api-tokens"
import type { ApiTokenResponse, TokenScope } from "@/api/schema"

const NAME_RULE = /^[a-z0-9-]+$/

const GRANTS: Array<{
  scope: TokenScope
  title: string
  describes: string
}> = [
  {
    scope: "readJournal",
    title: "Read the journal",
    describes:
      "Entries, activities, the calendar, statistics, correlations, cycle records and daily metrics.",
  },
  {
    scope: "writeJournal",
    title: "Write to the journal",
    describes:
      "Create, change and delete entries, activities, cycle records, reminders and media.",
  },
  {
    scope: "writeMetrics",
    title: "Write daily metrics",
    describes:
      "For an importer. Its writes are attributed to the token's own name.",
  },
  {
    scope: "readProfile",
    title: "Read the profile",
    describes: "The account's name, timezone, preferences and connections.",
  },
]

export function ApiTokensCard() {
  const { data: existing } = useApiTokens()
  const mint = useMintApiToken()
  const revoke = useRevokeApiToken()

  const [name, setName] = useState("")
  const [granted, setGranted] = useState<TokenScope[]>(["writeMetrics"])
  const [justMinted, setJustMinted] = useState<string | null>(null)

  const normalised = name.trim().toLowerCase()
  const namedWell = normalised.length > 0 && NAME_RULE.test(normalised)
  const canMint = namedWell && granted.length > 0

  const toggle = (scope: TokenScope, wanted: boolean) =>
    setGranted((held) =>
      wanted ? [...held, scope] : held.filter((one) => one !== scope)
    )

  const submit = () => {
    mint.mutate(
      { name: normalised, scopes: granted },
      {
        onSuccess: (minted) => {
          setJustMinted(minted.secret)
          setName("")
          setGranted(["writeMetrics"])
        },
        onError: (error) =>
          toast.add({
            title: error instanceof Error ? error.message : "Could not mint",
            type: "error",
          }),
      }
    )
  }

  return (
    <CollapsibleCard
      title="Access tokens"
      description="For automations and other clients — a widget, a Shortcut, a cron script."
      summary={
        existing?.length ? (
          <Badge variant="secondary">{existing.length}</Badge>
        ) : undefined
      }
    >
      <FieldGroup>
        {justMinted && (
          <Alert>
            <AlertTitle>Copy this now — it is not shown again</AlertTitle>
            <AlertDescription>
              <code className="break-all">{justMinted}</code>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setJustMinted(null)}
              >
                Done
              </Button>
            </AlertDescription>
          </Alert>
        )}

        <Field data-invalid={normalised.length > 0 && !namedWell}>
          <FieldLabel htmlFor="token-name">Name</FieldLabel>
          <Input
            id="token-name"
            placeholder="plasma-widget"
            value={name}
            aria-invalid={normalised.length > 0 && !namedWell}
            onChange={(event) => setName(event.target.value)}
          />
          <FieldDescription>
            Becomes the source recorded on any metrics it writes. Lowercase
            letters, digits and hyphens.
          </FieldDescription>
        </Field>

        <FieldSet>
          <FieldLegend variant="label">What it may do</FieldLegend>
          <FieldGroup className="gap-3">
            {GRANTS.map((grant) => (
              <Field key={grant.scope} orientation="horizontal">
                <Checkbox
                  id={`scope-${grant.scope}`}
                  checked={granted.includes(grant.scope)}
                  onCheckedChange={(wanted) =>
                    toggle(grant.scope, wanted === true)
                  }
                />
                <FieldContent>
                  <FieldLabel htmlFor={`scope-${grant.scope}`}>
                    {grant.title}
                  </FieldLabel>
                  <FieldDescription>{grant.describes}</FieldDescription>
                </FieldContent>
              </Field>
            ))}
          </FieldGroup>
        </FieldSet>

        <Field orientation="horizontal">
          <Button disabled={!canMint || mint.isPending} onClick={submit}>
            {mint.isPending ? "Minting…" : "Mint token"}
          </Button>
          {granted.length === 0 && (
            <FieldDescription>
              Tick at least one thing for it to do.
            </FieldDescription>
          )}
        </Field>

        {existing?.length ? (
          <FieldGroup className="gap-3">
            {existing.map((token) => (
              <TokenRow
                key={token.id}
                token={token}
                onRevoke={() => revoke.mutate(token.id)}
              />
            ))}
          </FieldGroup>
        ) : (
          <FieldDescription>No tokens yet.</FieldDescription>
        )}
      </FieldGroup>
    </CollapsibleCard>
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
    <Field orientation="horizontal">
      <FieldContent>
        <FieldTitleRow name={token.name} scopes={token.scopes} />
        <FieldDescription>{lastUsed}</FieldDescription>
      </FieldContent>
      <Button variant="ghost" size="icon" onClick={onRevoke}>
        <Trash2 className="text-destructive" />
      </Button>
    </Field>
  )
}

function FieldTitleRow({
  name,
  scopes,
}: {
  name: string
  scopes: TokenScope[]
}) {
  return (
    <div className="flex flex-wrap items-center gap-1.5">
      <span className="truncate text-xs font-medium">{name}</span>
      {scopes.map((scope) => (
        <Badge key={scope} variant="secondary">
          {scope}
        </Badge>
      ))}
    </div>
  )
}
