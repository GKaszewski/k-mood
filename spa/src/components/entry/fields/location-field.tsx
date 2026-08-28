import { useState } from "react"
import { Button } from "@/components/ui/button"
import { toast } from "@/components/ui/toast"
import type { Coordinates } from "@/lib/entry-draft"

interface LocationFieldProps {
  value: Coordinates | null
  onChange: (location: Coordinates | null) => void
}

const COORDINATE_PRECISION = 4

export function LocationField({ value, onChange }: LocationFieldProps) {
  const [locating, setLocating] = useState(false)

  const capture = () => {
    if (!("geolocation" in navigator)) {
      toast.add({
        title: "Location is unavailable on this device",
        type: "error",
      })
      return
    }

    setLocating(true)
    navigator.geolocation.getCurrentPosition(
      (position) => {
        onChange({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        })
        setLocating(false)
      },
      () => {
        toast.add({ title: "Could not read your location", type: "error" })
        setLocating(false)
      }
    )
  }

  return (
    <div className="flex flex-wrap items-center gap-3">
      <Button
        type="button"
        variant="outline"
        size="sm"
        onClick={capture}
        disabled={locating}
      >
        {locating ? "Locating…" : value ? "Update location" : "Add location"}
      </Button>

      {value && (
        <>
          <span className="text-xs text-muted-foreground tabular-nums">
            {value.latitude.toFixed(COORDINATE_PRECISION)},{" "}
            {value.longitude.toFixed(COORDINATE_PRECISION)}
          </span>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={() => onChange(null)}
          >
            Remove
          </Button>
        </>
      )}
    </div>
  )
}
