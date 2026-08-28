import { Badge } from "@/components/ui/badge"
import { CollapsibleCard } from "@/components/settings/collapsible-card"
import { ServerPicker } from "@/components/settings/server-picker"
import { isServerChosen, serverUrl } from "@/api/server-url"
import { useServerInfo } from "@/hooks/use-server"

export function ServerCard() {
  const { data: info } = useServerInfo()
  const elsewhere = isServerChosen()

  return (
    <CollapsibleCard
      title="Server"
      description="This app talks to whichever k-mood server you point it at."
      summary={
        <Badge variant={elsewhere ? "secondary" : "outline"}>
          {elsewhere ? serverUrl() : "this site"}
        </Badge>
      }
    >
      <ServerPicker version={info?.version} />
    </CollapsibleCard>
  )
}
