import { createFileRoute } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useExport } from "@/hooks/use-import-export"
import { Download, FileArchive } from "lucide-react"

export const Route = createFileRoute("/_auth/export")({
  component: ExportPage,
})

function ExportPage() {
  const exportData = useExport()

  return (
    <div className="flex flex-col gap-6 py-6">
      <h1 className="text-xl font-bold">Export data</h1>

      <Card>
        <CardHeader>
          <CardTitle className="text-sm">What's included</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-2 text-sm text-muted-foreground">
          <p>Your export will contain:</p>
          <ul className="list-inside list-disc space-y-1">
            <li>All mood entries with notes</li>
            <li>Your activity catalog</li>
            <li>Photos and voice memos</li>
            <li>Reminder schedules</li>
          </ul>
          <p className="mt-2">
            The export is a ZIP file in k-mood's native format. You can
            re-import it into any k-mood instance.
          </p>
        </CardContent>
      </Card>

      <Button
        size="lg"
        onClick={() => exportData.mutate()}
        disabled={exportData.isPending}
        className="w-full"
      >
        {exportData.isPending ? (
          <>Preparing export...</>
        ) : (
          <>
            <Download className="mr-2 h-4 w-4" />
            Download export
          </>
        )}
      </Button>

      {exportData.isSuccess && (
        <Card>
          <CardContent className="flex items-center gap-3 py-4">
            <FileArchive className="h-5 w-5 text-green-500" />
            <span className="text-sm">Export downloaded successfully</span>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
