import { messageFor } from "@/lib/api-message"

export function FormError({ error }: { error: unknown }) {
  if (!error) return null

  return (
    <p role="alert" className="text-sm text-destructive">
      {messageFor(error)}
    </p>
  )
}
