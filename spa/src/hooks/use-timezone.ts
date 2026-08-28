import { useEffect } from "react"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { users } from "@/api/client"
import { CURRENT_USER_KEY, useAuth } from "./use-auth"

function platformTimezone(): string | null {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || null
  } catch {
    return null
  }
}

/**
 * Adopts the device's timezone the first time an account arrives without one.
 * Reminders fire against the stored zone, so a missing one is worth filling in
 * without asking.
 */
export function useEnsureTimezone() {
  const queryClient = useQueryClient()
  const { user } = useAuth()

  const adopt = useMutation({
    meta: { silent: true },
    mutationFn: (timezone: string) => users.updateProfile({ timezone }),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: CURRENT_USER_KEY }),
  })

  const missing = user !== null && user.timezone === null
  const detected = platformTimezone()
  const { mutate: adoptTimezone, isPending, isSuccess } = adopt

  useEffect(() => {
    if (!missing || !detected || isPending || isSuccess) return
    adoptTimezone(detected)
  }, [missing, detected, isPending, isSuccess, adoptTimezone])
}
