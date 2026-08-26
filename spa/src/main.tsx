import ReactDOM from "react-dom/client"
import { RouterProvider } from "@tanstack/react-router"
import {
  MutationCache,
  QueryClient,
  QueryClientProvider,
} from "@tanstack/react-query"
import { getRouter } from "./router"
import { toast, Toaster } from "@/components/ui/toast"
import { ApiError } from "@/api/client"
import "./styles.css"

const queryClient = new QueryClient({
  mutationCache: new MutationCache({
    onSuccess: (_data, _vars, _ctx, mutation) => {
      const message = mutation.meta?.success
      if (typeof message === "string") {
        toast.add({ title: message, type: "success" })
      }
    },
    onError: (error, _vars, _ctx, mutation) => {
      if (mutation.meta?.silent) return
      const message =
        error instanceof ApiError ? error.message : "Something went wrong"
      toast.add({ title: message, type: "error" })
    },
  }),
})

const router = getRouter()

ReactDOM.createRoot(document.getElementById("root")!).render(
  <QueryClientProvider client={queryClient}>
    <Toaster>
      <RouterProvider router={router} />
    </Toaster>
  </QueryClientProvider>
)

if ("serviceWorker" in navigator) {
  navigator.serviceWorker.register("/sw.js")
}
