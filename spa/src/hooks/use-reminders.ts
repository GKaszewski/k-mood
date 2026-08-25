import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { reminders } from "@/api/client"
import type { CreateReminderRequest, UpdateReminderRequest } from "@/api/schema"

export function useReminders() {
  return useQuery({
    queryKey: ["reminders"],
    queryFn: () => reminders.list(),
  })
}

export function useCreateReminder() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (body: CreateReminderRequest) => reminders.create(body),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["reminders"] }),
    meta: { success: "Reminder created" },
  })
}

export function useUpdateReminder() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ id, body }: { id: string; body: UpdateReminderRequest }) =>
      reminders.update(id, body),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["reminders"] }),
    meta: { success: "Reminder updated" },
  })
}

export function useDeleteReminder() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => reminders.delete(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["reminders"] }),
    meta: { success: "Reminder deleted" },
  })
}
