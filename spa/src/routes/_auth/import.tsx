import { useState, useRef } from "react"
import { createFileRoute } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useImport, useRestore } from "@/hooks/use-import-export"
import { ImportWizard } from "@/components/import/import-wizard"
import { WizardResult } from "@/components/import/wizard-result"
import { Upload, FileUp } from "lucide-react"
import type { ImportResultResponse, RestoreOutcomeResponse } from "@/api/schema"

export const Route = createFileRoute("/_auth/import")({
  component: ImportPage,
})

type ImportSource = "daylio" | "kmood" | "csv"

function ImportPage() {
  const [source, setSource] = useState<ImportSource | null>(null)
  const [file, setFile] = useState<File | null>(null)
  const [result, setResult] = useState<ImportResultResponse | null>(null)
  const [restored, setRestored] = useState<RestoreOutcomeResponse | null>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)
  const importData = useImport()
  const restoreBackup = useRestore()
  const working = importData.isPending || restoreBackup.isPending

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selected = e.target.files?.[0]
    if (selected) {
      setFile(selected)
      setResult(null)
      setRestored(null)
    }
  }

  const handleImport = () => {
    if (!file) return

    if (source === "kmood") {
      restoreBackup.mutate(file, { onSuccess: setRestored })
      return
    }

    importData.mutate(file, { onSuccess: setResult })
  }

  return (
    <div className="flex flex-col gap-6 py-6">
      <h1 className="text-xl font-bold">Import data</h1>

      {!source && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Choose source</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            <SourceOption
              label="Daylio"
              description="Import from Daylio CSV export"
              onClick={() => setSource("daylio")}
            />
            <SourceOption
              label="k-mood backup"
              description="Restore a complete backup, with everything in it"
              onClick={() => setSource("kmood")}
            />
            <SourceOption
              label="Generic CSV"
              description="Import from any CSV/TSV with a mapping wizard"
              onClick={() => setSource("csv")}
            />
          </CardContent>
        </Card>
      )}

      {(source === "daylio" || source === "kmood") && !result && !restored && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">
              {source === "daylio"
                ? "Upload Daylio export"
                : "Upload k-mood backup"}
            </CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <p className="text-sm text-muted-foreground">
              {source === "daylio"
                ? "Export your data from Daylio (Settings → Export → CSV) and upload the file here. Daylio records the time you logged but not which timezone you were in, so times are read as your own — 8 PM in the file becomes 8 PM where you live. Set your timezone in settings first."
                : "Upload a complete backup to restore it. Everything in it comes back: entries with every dimension, daily metrics, cycle records, activities, reminders and settings. A restore adds to this account rather than replacing it."}
            </p>

            <input
              ref={fileInputRef}
              type="file"
              accept={source === "kmood" ? ".zip" : ".csv,.tsv,.txt"}
              onChange={handleFileChange}
              className="hidden"
            />

            <Button
              variant="outline"
              onClick={() => fileInputRef.current?.click()}
              className="w-full"
            >
              <FileUp className="mr-2 h-4 w-4" />
              {file ? file.name : "Choose file"}
            </Button>

            {file && (
              <Button
                size="lg"
                onClick={handleImport}
                disabled={working}
                className="w-full"
              >
                {working ? (
                  source === "kmood" ? (
                    "Restoring..."
                  ) : (
                    "Importing..."
                  )
                ) : (
                  <>
                    <Upload className="mr-2 h-4 w-4" />
                    {source === "kmood" ? "Restore" : "Import"}
                  </>
                )}
              </Button>
            )}
          </CardContent>
        </Card>
      )}

      {source === "csv" && !result && <ImportWizard />}

      {result && <WizardResult result={result} />}

      {restored && <RestoreSummary outcome={restored} />}

      {source && !result && !restored && (
        <Button
          variant="ghost"
          onClick={() => {
            setSource(null)
            setFile(null)
          }}
        >
          ← Back to source selection
        </Button>
      )}
    </div>
  )
}

function SourceOption({
  label,
  description,
  onClick,
}: {
  label: string
  description: string
  onClick: () => void
}) {
  return (
    <Button
      variant="ghost"
      onClick={onClick}
      className="flex h-auto flex-col items-start gap-0.5 p-4 text-left"
    >
      <span className="text-sm font-medium">{label}</span>
      <span className="text-xs text-muted-foreground">{description}</span>
    </Button>
  )
}

function RestoreSummary({ outcome }: { outcome: RestoreOutcomeResponse }) {
  const counts = [
    ["Entries", outcome.entries],
    ["Daily metrics", outcome.metrics],
    ["Cycle records", outcome.cycleStarts],
    ["Activities", outcome.activities],
    ["Reminders", outcome.reminders],
    ["Photos and voice memos", outcome.media],
  ] as const

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm">Restored</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        {counts.map(([label, count]) => (
          <div key={label} className="flex justify-between text-sm">
            <span className="text-muted-foreground">{label}</span>
            <span className="font-medium tabular-nums">{count}</span>
          </div>
        ))}

        {outcome.unreadable.length > 0 && (
          <div className="mt-2 flex flex-col gap-1 border-l-2 border-destructive/40 pl-2">
            <span className="text-xs font-medium">
              Could not be read, and was not restored:
            </span>
            {outcome.unreadable.map((reason) => (
              <span key={reason} className="text-[10px] text-muted-foreground">
                {reason}
              </span>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
