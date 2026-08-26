import {
  Cloud,
  CloudDrizzle,
  CloudFog,
  CloudLightning,
  CloudRain,
  Snowflake,
  Sun,
} from "lucide-react"
import type { LucideIcon } from "lucide-react"
import { weatherOf } from "@/lib/dimensions"
import type { EntryResponse } from "@/api/schema"

type Condition = NonNullable<ReturnType<typeof weatherOf>>["condition"]

const LOOKS_LIKE: Record<Condition, { icon: LucideIcon; label: string }> = {
  clear: { icon: Sun, label: "Clear" },
  cloudy: { icon: Cloud, label: "Cloudy" },
  fog: { icon: CloudFog, label: "Fog" },
  drizzle: { icon: CloudDrizzle, label: "Drizzle" },
  rain: { icon: CloudRain, label: "Rain" },
  snow: { icon: Snowflake, label: "Snow" },
  thunderstorm: { icon: CloudLightning, label: "Thunderstorm" },
}

export function EntryWeather({ entry }: { entry: EntryResponse }) {
  const weather = weatherOf(entry)
  if (!weather) return null

  const { icon: Icon, label } = LOOKS_LIKE[weather.condition]

  return (
    <span
      className="flex items-center gap-1.5 text-xs text-muted-foreground"
      title={`Observed by ${weather.observedBy}`}
    >
      <Icon className="h-3.5 w-3.5" />
      {label}, {Math.round(weather.temperature)}°C
    </span>
  )
}
