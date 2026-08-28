import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerHeader,
  DrawerTitle,
} from "@/components/ui/drawer"
import { EntryForm } from "./entry-form"
import { useNewEntryForm } from "@/hooks/use-new-entry-form"

interface LogSheetProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

/**
 * Logging a mood from wherever you are. The same form the page shows, in front
 * of what you were already looking at, because the common case is one tap.
 */
export function LogSheet({ open, onOpenChange }: LogSheetProps) {
  const { controls, submit, isSaving } = useNewEntryForm(() =>
    onOpenChange(false)
  )

  return (
    <Drawer open={open} onOpenChange={onOpenChange} showSwipeHandle>
      <DrawerContent className="mx-auto max-h-[85svh] w-full max-w-md">
        <DrawerHeader className="sr-only">
          <DrawerTitle>Log a mood</DrawerTitle>
          <DrawerDescription>
            Pick how you feel, and add as much or as little else as you want.
          </DrawerDescription>
        </DrawerHeader>

        <div className="overflow-y-auto px-4 pt-2 pb-6 pb-safe">
          <EntryForm
            controls={controls}
            title="How are you feeling?"
            submitLabel="Save entry"
            isSaving={isSaving}
            onSubmit={submit}
          />
        </div>
      </DrawerContent>
    </Drawer>
  )
}
