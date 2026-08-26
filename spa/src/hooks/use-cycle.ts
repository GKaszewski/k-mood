import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { cycle } from "@/api/client"

const CYCLE = ["cycle"]
const PREFERENCES = ["preferences"]

export function usePreferences() {
  return useQuery({
    queryKey: PREFERENCES,
    queryFn: () => cycle.preferences(),
  })
}

export function useSetCycleTracking() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (tracksCycle: boolean) => cycle.setTracking(tracksCycle),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PREFERENCES })
      queryClient.invalidateQueries({ queryKey: CYCLE })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useCycle(enabled: boolean) {
  return useQuery({
    queryKey: CYCLE,
    queryFn: () => cycle.read(),
    enabled,
  })
}

export function useRecordCycleStart() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (date: string) => cycle.record(date),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CYCLE })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useForgetCycleStart() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (date: string) => cycle.forget(date),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CYCLE })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}
