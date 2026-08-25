import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { MoodBadge } from "@/components/mood/mood-badge"
import { useImport } from "@/hooks/use-import-export"
import type { ImportResultResponse } from "@/api/schema"
import type { ColumnMapping, ParsedFile } from "./import-wizard"

interface WizardPreviewProps {
  file: File
  parsed: ParsedFile
  mapping: ColumnMapping
  onDone: (result: ImportResultResponse) => void
  onBack: () => void
}

export function WizardPreview({
  file,
  parsed,
  mapping,
  onDone,
  onBack,
}: WizardPreviewProps) {
  const importData = useImport()

  const preview = parsed.rows.slice(0, 5).map((row) => ({
    date: mapping.date !== null ? (row[mapping.date] ?? "") : "",
    time: mapping.time !== null ? (row[mapping.time] ?? "") : "",
    mood: mapping.mood !== null ? (row[mapping.mood] ?? "") : "",
    activities:
      mapping.activities !== null ? (row[mapping.activities] ?? "") : "",
    note: mapping.note !== null ? (row[mapping.note] ?? "") : "",
  }))

  const moodValue = (mood: string): number => {
    const num = parseInt(mood)
    if (num >= 1 && num <= 5) return num
    const map: Record<string, number> = {
      awful: 1,
      bad: 2,
      meh: 3,
      good: 4,
      rad: 5,
    }
    return map[mood.toLowerCase()] ?? 3
  }

  const handleImport = () => {
    importData.mutate(file, {
      onSuccess: (result) => onDone(result),
    })
  }

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={onBack}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <h2 className="text-lg font-bold">Preview</h2>
      </div>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">First {preview.length} rows</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          {preview.map((row, i) => (
            <div
              key={i}
              className="flex items-start gap-3 rounded-lg border p-3 text-sm"
            >
              <MoodBadge value={moodValue(row.mood)} />
              <div className="min-w-0 flex-1">
                <p className="font-medium">
                  {row.date} {row.time}
                </p>
                {row.activities && (
                  <p className="text-xs text-muted-foreground">
                    {row.activities}
                  </p>
                )}
                {row.note && (
                  <p className="mt-1 line-clamp-1 text-xs text-muted-foreground">
                    {row.note}
                  </p>
                )}
              </div>
            </div>
          ))}
        </CardContent>
      </Card>

      <Button onClick={handleImport} disabled={importData.isPending}>
        {importData.isPending ? "Importing..." : "Confirm import"}
      </Button>
    </div>
  )
}
