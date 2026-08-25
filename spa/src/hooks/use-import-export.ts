import { useMutation, useQueryClient } from "@tanstack/react-query"
import { data } from "@/api/client"

export function useExport() {
  return useMutation({
    meta: { success: "Export downloaded" },
    mutationFn: () => data.export(),
    onSuccess: (blob) => {
      const url = URL.createObjectURL(blob)
      const a = document.createElement("a")
      a.href = url
      a.download = `k-mood-export-${new Date().toISOString().slice(0, 10)}.zip`
      a.click()
      URL.revokeObjectURL(url)
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
