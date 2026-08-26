import {
  Activity,
  BedDouble,
  EyeOff,
  Footprints,
  HeartPulse,
  Smartphone,
  Timer,
  Wine,
} from "lucide-react"
import type { LucideIcon } from "lucide-react"
import type { MetricKind } from "@/api/schema"

export type MetricField = {
  kind: MetricKind
  label: string
  unit: string
  min: number
  max: number
  icon: LucideIcon
}

export const METRIC_FIELDS: MetricField[] = [
  {
    kind: "steps",
    label: "Steps",
    unit: "steps",
    min: 0,
    max: 200000,
    icon: Footprints,
  },
  {
    kind: "sleepMinutes",
    label: "Sleep",
    unit: "minutes",
    min: 0,
    max: 1440,
    icon: BedDouble,
  },
  {
    kind: "awakeMinutes",
    label: "Awake in bed",
    unit: "minutes",
    min: 0,
    max: 1440,
    icon: EyeOff,
  },
  {
    kind: "restingHeartRate",
    label: "Resting heart rate",
    unit: "bpm",
    min: 25,
    max: 120,
    icon: HeartPulse,
  },
  {
    kind: "hrv",
    label: "HRV",
    unit: "ms",
    min: 1,
    max: 300,
    icon: Activity,
  },
  {
    kind: "exerciseMinutes",
    label: "Exercise",
    unit: "minutes",
    min: 0,
    max: 1440,
    icon: Timer,
  },
  {
    kind: "screenTimeMinutes",
    label: "Screen time",
    unit: "minutes",
    min: 0,
    max: 1440,
    icon: Smartphone,
  },
  {
    kind: "alcoholicDrinks",
    label: "Alcoholic drinks",
    unit: "standard drinks",
    min: 0,
    max: 60,
    icon: Wine,
  },
]
