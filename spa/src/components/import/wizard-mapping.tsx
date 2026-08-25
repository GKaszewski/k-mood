import { ArrowLeft } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import type { ColumnMapping } from "./import-wizard"

const DOMAIN_FIELDS = [
  { value: "-1", label: "Skip" },
  { value: "date", label: "Date" },
  { value: "time", label: "Time" },
  { value: "mood", label: "Mood" },
  { value: "activities", label: "Activities" },
  { value: "note", label: "Note" },
]

interface WizardMappingProps {
  headers: string[]
  mapping: ColumnMapping
  onChange: (mapping: ColumnMapping) => void
  onNext: () => void
  onBack: () => void
}

export function WizardMapping({
  headers,
  mapping,
  onChange,
  onNext,
  onBack,
}: WizardMappingProps) {
  const setField = (field: keyof ColumnMapping, colIndex: number) => {
    const newMapping = { ...mapping }
    for (const key of Object.keys(newMapping) as (keyof ColumnMapping)[]) {
      if (newMapping[key] === colIndex) newMapping[key] = null
    }
    if (colIndex >= 0) newMapping[field] = colIndex
    onChange(newMapping)
  }

  const getFieldForColumn = (colIndex: number): string => {
    for (const [field, idx] of Object.entries(mapping)) {
      if (idx === colIndex) return field
    }
    return "-1"
  }

  const hasMood = mapping.mood !== null && mapping.mood >= 0
  const hasDate = mapping.date !== null && mapping.date >= 0

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={onBack}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <h2 className="text-lg font-bold">Map columns</h2>
      </div>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm">
            Assign each column to a field
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          {headers.map((header, i) => (
            <div key={i} className="flex items-center justify-between gap-3">
              <span className="min-w-0 truncate text-sm font-medium">
                {header}
              </span>
              <select
                value={getFieldForColumn(i)}
                onChange={(e) => {
                  const val = e.target.value
                  if (val === "-1") {
                    const current = getFieldForColumn(i)
                    if (current !== "-1") {
                      onChange({ ...mapping, [current]: null })
                    }
                  } else {
                    setField(val as keyof ColumnMapping, i)
                  }
                }}
                className="h-9 w-32 shrink-0 rounded-md border bg-transparent px-2 text-sm"
              >
                {DOMAIN_FIELDS.map((f) => (
                  <option key={f.value} value={f.value}>
                    {f.label}
                  </option>
                ))}
              </select>
            </div>
          ))}
        </CardContent>
      </Card>

      <Button onClick={onNext} disabled={!hasMood || !hasDate}>
        {!hasMood || !hasDate ? "Map at least Date and Mood" : "Preview import"}
      </Button>
    </div>
  )
}
