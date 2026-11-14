import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import type { GroupOrder } from "@/lib/compare-groups"

interface SortToggleProps {
  value: GroupOrder
  onChange: (order: GroupOrder) => void
}

export function SortToggle({ value, onChange }: SortToggleProps) {
  return (
    <ToggleGroup
      size="sm"
      variant="outline"
      spacing={0}
      value={[value]}
      onValueChange={(next: string[]) => {
        const picked = next[0] as GroupOrder | undefined
        if (picked) onChange(picked)
      }}
      aria-label="Order rows"
    >
      <ToggleGroupItem value="server">Usual order</ToggleGroupItem>
      <ToggleGroupItem value="change">Biggest change</ToggleGroupItem>
    </ToggleGroup>
  )
}
