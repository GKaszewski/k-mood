import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { auth, users } from "@/api/client"
import { useNavigate } from "@tanstack/react-router"

export function useAuth() {
  const { data: user, isLoading } = useQuery({
    queryKey: ["auth", "me"],
    queryFn: async () => {
      const token = localStorage.getItem("accessToken")
      if (!token) return null

      try {
        return await users.me()
      } catch {
        return null
      }
    },
    retry: false,
    staleTime: 5 * 60 * 1000,
  })

  return {
    user: user ?? null,
    isAuthenticated: !!user,
    isLoading,
  }
}

export function useLogin() {
  const queryClient = useQueryClient()
  const navigate = useNavigate()

  return useMutation({
    meta: { success: "Welcome back!", silent: false },
    mutationFn: (body: { identifier: string; password: string }) =>
      auth.login(body),
    onSuccess: (data) => {
      localStorage.setItem("accessToken", data.accessToken)
      localStorage.setItem("refreshToken", data.refreshToken)
      queryClient.invalidateQueries({ queryKey: ["auth"] })
      navigate({ to: "/" })
    },
  })
}

export function useRegister() {
  const navigate = useNavigate()

  return useMutation({
    meta: { success: "Account created! Please log in." },
    mutationFn: (body: { username: string; email: string; password: string }) =>
      auth.register(body),
    onSuccess: () => {
      navigate({ to: "/login" })
    },
  })
}

export function useLogout() {
  const queryClient = useQueryClient()
  const navigate = useNavigate()

  return useMutation({
    meta: { silent: true },
    mutationFn: async () => {
      const refreshToken = localStorage.getItem("refreshToken")
      if (refreshToken) {
        await auth.logout(refreshToken)
      }
    },
    onSettled: () => {
      localStorage.removeItem("accessToken")
      localStorage.removeItem("refreshToken")
      queryClient.clear()
      navigate({ to: "/login" })
    },
  })
}
