import "@tanstack/react-query"

declare module "@tanstack/react-query" {
  interface Register {
    mutationMeta: {
      success?: string
      silent?: boolean
    }
  }
}
