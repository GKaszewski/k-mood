import { useRef } from "react"
import { Upload } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import type { ParsedFile } from "./import-wizard"

interface WizardUploadProps {
  onParsed: (file: File, data: ParsedFile) => void
}

export function WizardUpload({ onParsed }: WizardUploadProps) {
  const fileRef = useRef<HTMLInputElement>(null)

  const handleFile = (file: File) => {
    const reader = new FileReader()
    reader.onload = () => {
      const text = reader.result as string
      const parsed = parseCSV(text)
      onParsed(file, parsed)
    }
    reader.readAsText(file)
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    const file = e.dataTransfer.files[0]
    if (file) handleFile(file)
  }

  return (
    <Card>
      <CardContent>
        <div
          onDrop={handleDrop}
          onDragOver={(e) => e.preventDefault()}
          onClick={() => fileRef.current?.click()}
          className="flex cursor-pointer flex-col items-center gap-3 rounded-xl border-2 border-dashed border-muted-foreground/30 p-10 text-center"
        >
          <Upload className="h-8 w-8 text-muted-foreground" />
          <p className="text-sm font-medium">Drop your CSV file here</p>
          <p className="text-xs text-muted-foreground">or click to browse</p>
          <input
            ref={fileRef}
            type="file"
            accept=".csv,.tsv,.txt"
            className="hidden"
            onChange={(e) => {
              const file = e.target.files?.[0]
              if (file) handleFile(file)
            }}
          />
        </div>
      </CardContent>
    </Card>
  )
}

function parseCSV(text: string): ParsedFile {
  const lines = text.trim().split("\n")
  if (lines.length === 0) return { headers: [], rows: [], delimiter: "," }

  const delimiter = lines[0].includes("\t") ? "\t" : ","
  const headers = parseLine(lines[0], delimiter)
  const rows = lines.slice(1, 6).map((line) => parseLine(line, delimiter))

  return { headers, rows, delimiter }
}

function parseLine(line: string, delimiter: string): string[] {
  const result: string[] = []
  let current = ""
  let inQuotes = false

  for (const char of line) {
    if (char === '"') {
      inQuotes = !inQuotes
    } else if (char === delimiter && !inQuotes) {
      result.push(current.trim())
      current = ""
    } else {
      current += char
    }
  }
  result.push(current.trim())

  return result
}
