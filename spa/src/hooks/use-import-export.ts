import { useMutation, useQueryClient } from "@tanstack/react-query"
import { data } from "@/api/client"

function today(): string {
  return new Date().toISOString().slice(0, 10)
}

function save(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}

export function useCompleteBackup() {
  return useMutation({
    meta: { success: "Complete backup downloaded" },
    mutationFn: () => data.completeBackup(),
    onSuccess: (blob) => save(blob, `k-mood-complete-backup-${today()}.zip`),
  })
}

export function useShareableExtract() {
  return useMutation({
    meta: { success: "Shareable journal downloaded" },
    mutationFn: () => data.shareableExtract(),
    onSuccess: (blob) => save(blob, `k-mood-shareable-journal-${today()}.md`),
  })
}

export function useRestore() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Backup restored" },
    mutationFn: (file: File) => data.restore(file),
    onSuccess: () => {
      for (const key of [
        "entries",
        "activities",
        "stats",
        "calendar",
        "daily-metrics",
        "cycle",
        "preferences",
      ]) {
        queryClient.invalidateQueries({ queryKey: [key] })
      }
    },
  })
}

export function useImport() {
  const queryClient = useQueryClient()

  return useMutation({
    meta: { success: "Data imported successfully" },
    mutationFn: (file: File) => data.import(file),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["entries"] })
      queryClient.invalidateQueries({ queryKey: ["activities"] })
      queryClient.invalidateQueries({ queryKey: ["stats"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}
