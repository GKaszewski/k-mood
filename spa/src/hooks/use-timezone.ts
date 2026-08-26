import { useEffect } from "react"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { users } from "@/api/client"

function platformTimezone(): string | null {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || null
  } catch {
    return null
  }
}

export function useEnsureTimezone() {
  const queryClient = useQueryClient()

  const profile = useQuery({
    queryKey: ["me"],
    queryFn: () => users.me(),
  })

  const adopt = useMutation({
    mutationFn: (timezone: string) => users.updateProfile({ timezone }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["me"] }),
  })

  const missing = profile.data !== undefined && profile.data.timezone === null
  const detected = platformTimezone()

  useEffect(() => {
    if (!missing || !detected || adopt.isPending || adopt.isSuccess) return
    adopt.mutate(detected)
  }, [missing, detected, adopt])
}
