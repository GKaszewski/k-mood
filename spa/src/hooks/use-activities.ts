import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { activities } from "@/api/client"
import type { CreateActivityRequest } from "@/api/schema"

export function useActivities() {
  return useQuery({
    queryKey: ["activities"],
    queryFn: () => activities.list(),
  })
}

export function useCreateActivity() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Activity created" },
    mutationFn: (body: CreateActivityRequest) => activities.create(body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["activities"] })
    },
  })
}
