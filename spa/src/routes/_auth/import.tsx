import { useState, useRef } from "react"
import { createFileRoute } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useImport } from "@/hooks/use-import-export"
import { ImportWizard } from "@/components/import/import-wizard"
import { WizardResult } from "@/components/import/wizard-result"
import { Upload, FileUp } from "lucide-react"
import type { ImportResultResponse } from "@/api/schema"

export const Route = createFileRoute("/_auth/import")({
  component: ImportPage,
})

type ImportSource = "daylio" | "kmood" | "csv"

function ImportPage() {
  const [source, setSource] = useState<ImportSource | null>(null)
  const [file, setFile] = useState<File | null>(null)
  const [result, setResult] = useState<ImportResultResponse | null>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)
  const importData = useImport()

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selected = e.target.files?.[0]
    if (selected) {
      setFile(selected)
      setResult(null)
    }
  }

  const handleImport = () => {
    if (!file) return
    importData.mutate(file, {
      onSuccess: (data) => setResult(data),
    })
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
              description="Restore from a k-mood ZIP export"
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

      {(source === "daylio" || source === "kmood") && !result && (
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
                ? "Export your data from Daylio (Settings → Export → CSV) and upload the file here."
                : "Upload a previously exported k-mood ZIP file to restore your data."}
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
                disabled={importData.isPending}
                className="w-full"
              >
                {importData.isPending ? (
                  "Importing..."
                ) : (
                  <>
                    <Upload className="mr-2 h-4 w-4" />
                    Import
                  </>
                )}
              </Button>
            )}
          </CardContent>
        </Card>
      )}

      {source === "csv" && !result && <ImportWizard />}

      {result && <WizardResult result={result} />}

      {source && !result && (
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
