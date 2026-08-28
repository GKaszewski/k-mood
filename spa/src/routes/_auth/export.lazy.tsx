import { createLazyFileRoute } from "@tanstack/react-router"
import { AlertTriangle, Download, Share2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  useCompleteBackup,
  useShareableExtract,
} from "@/hooks/use-import-export"

export const Route = createLazyFileRoute("/_auth/export")({
  component: ExportPage,
})

function ExportPage() {
  const backup = useCompleteBackup()
  const extract = useShareableExtract()

  return (
    <div className="flex flex-col gap-6 py-6">
      <h1 className="text-xl font-bold">Take your data out</h1>

      <p className="text-sm text-muted-foreground">
        Two different things, for two different reasons. One is for keeping, the
        other is for handing to someone.
      </p>

      <Card className="border-destructive/40">
        <CardHeader className="pb-2">
          <CardTitle className="flex items-center gap-2 text-sm">
            <AlertTriangle className="h-4 w-4 text-destructive" />
            Complete backup — keep this private
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <p className="text-sm text-muted-foreground">
            Everything this account knows about you, so that nothing is lost if
            you need to restore it.
          </p>
          <ul className="list-inside list-disc space-y-0.5 text-xs text-muted-foreground">
            <li>Every entry, with its notes, photos and voice memos</li>
            <li>Where you were and what you were listening to</li>
            <li>Steps, sleep, heart rate and every other daily metric</li>
            <li>Cycle records, if you track them</li>
            <li>Your activity catalogue, reminders and settings</li>
          </ul>
          <p className="text-xs text-destructive">
            Do not send this to anyone. It is a copy of your whole journal.
          </p>
          <Button
            size="lg"
            variant="outline"
            className="w-full"
            disabled={backup.isPending}
            onClick={() => backup.mutate()}
          >
            <Download className="mr-2 h-4 w-4" />
            {backup.isPending
              ? "Preparing backup…"
              : "Download complete backup (.zip)"}
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="flex items-center gap-2 text-sm">
            <Share2 className="h-4 w-4" />
            Shareable journal — safe to send
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <p className="text-sm text-muted-foreground">
            A readable document of how you have been feeling, for a therapist, a
            doctor, or someone you trust.
          </p>
          <ul className="list-inside list-disc space-y-0.5 text-xs text-muted-foreground">
            <li>Your mood, day by day</li>
            <li>What you wrote</li>
            <li>What you tagged</li>
          </ul>
          <p className="text-xs text-muted-foreground">
            It carries no places, no health readings, no cycle records and no
            photos. It cannot be used to restore your account.
          </p>
          <Button
            size="lg"
            className="w-full"
            disabled={extract.isPending}
            onClick={() => extract.mutate()}
          >
            <Share2 className="mr-2 h-4 w-4" />
            {extract.isPending
              ? "Preparing journal…"
              : "Download shareable journal (.md)"}
          </Button>
        </CardContent>
      </Card>
    </div>
  )
}
