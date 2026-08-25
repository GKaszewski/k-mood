import { useMutation, useQueryClient } from "@tanstack/react-query"
import { activities, entries } from "@/api/client"

export function useRenameActivity() {
  const queryClient = useQueryClient()
  return useMutation({
    meta: { success: "Activity renamed" },
    mutationFn: ({ id, newName }: { id: string; newName: string }) =>
      activities.rename(id, { newName }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}

export function useSetCategory() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ id, category }: { id: string; category: string | null }) =>
      activities.setCategory(id, { category }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}

export function useArchiveActivity() {
  const queryClient = useQueryClient()
  return useMutation({
    meta: { success: "Activity archived" },
    mutationFn: (id: string) => activities.archive(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}

export function useUnarchiveActivity() {
  const queryClient = useQueryClient()
  return useMutation({
    meta: { success: "Activity restored" },
    mutationFn: (id: string) => activities.unarchive(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}

export function useDeleteActivity() {
  const queryClient = useQueryClient()
  return useMutation({
    meta: { success: "Activity deleted" },
    mutationFn: (id: string) => activities.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}

export function useReplaceActivity() {
  const queryClient = useQueryClient()
  return useMutation({
    meta: { success: "Activity merged" },
    mutationFn: ({
      oldActivityId,
      newActivityId,
    }: {
      oldActivityId: string
      newActivityId: string
    }) => entries.replaceActivity({ oldActivityId, newActivityId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
      queryClient.invalidateQueries({ queryKey: ["entries"] })
    },
  })
}
