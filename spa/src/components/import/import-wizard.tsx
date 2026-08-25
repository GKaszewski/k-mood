import { useState } from "react"
import type { ImportResultResponse } from "@/api/schema"
import { WizardUpload } from "./wizard-upload"
import { WizardMapping } from "./wizard-mapping"
import { WizardPreview } from "./wizard-preview"
import { WizardResult } from "./wizard-result"

export type ColumnMapping = {
  date: number | null
  time: number | null
  mood: number | null
  activities: number | null
  note: number | null
}

export type ParsedFile = {
  headers: string[]
  rows: string[][]
  delimiter: string
}

export function ImportWizard() {
  const [step, setStep] = useState(0)
  const [file, setFile] = useState<File | null>(null)
  const [parsed, setParsed] = useState<ParsedFile | null>(null)
  const [mapping, setMapping] = useState<ColumnMapping>({
    date: null,
    time: null,
    mood: null,
    activities: null,
    note: null,
  })
  const [result, setResult] = useState<ImportResultResponse | null>(null)

  const handleFileParsed = (f: File, data: ParsedFile) => {
    setFile(f)
    setParsed(data)
    autoDetectMapping(data.headers)
    setStep(1)
  }

  const autoDetectMapping = (headers: string[]) => {
    const lower = headers.map((h) => h.toLowerCase())
    setMapping({
      date: lower.findIndex((h) => h.includes("date") || h === "full_date"),
      time: lower.findIndex((h) => h.includes("time")),
      mood: lower.findIndex((h) => h.includes("mood") || h.includes("feeling")),
      activities: lower.findIndex(
        (h) => h.includes("activit") || h.includes("tag")
      ),
      note: lower.findIndex((h) => h.includes("note") || h.includes("comment")),
    })
  }

  const handleImportDone = (res: ImportResultResponse) => {
    setResult(res)
    setStep(3)
  }

  return (
    <div className="flex flex-col gap-4">
      <div className="flex gap-1">
        {[0, 1, 2, 3].map((s) => (
          <div
            key={s}
            className={`h-1 flex-1 rounded-full ${s <= step ? "bg-primary" : "bg-muted"}`}
          />
        ))}
      </div>

      {step === 0 && <WizardUpload onParsed={handleFileParsed} />}

      {step === 1 && parsed && (
        <WizardMapping
          headers={parsed.headers}
          mapping={mapping}
          onChange={setMapping}
          onNext={() => setStep(2)}
          onBack={() => setStep(0)}
        />
      )}

      {step === 2 && parsed && file && (
        <WizardPreview
          file={file}
          parsed={parsed}
          mapping={mapping}
          onDone={handleImportDone}
          onBack={() => setStep(1)}
        />
      )}

      {step === 3 && result && <WizardResult result={result} />}
    </div>
  )
}
