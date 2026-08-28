import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { useNavigate } from "@tanstack/react-router"
import { auth, users } from "@/api/client"
import {
  forgetSession,
  hasSession,
  refreshToken,
  rememberSession,
} from "@/api/session"

/**
 * The single read of the signed-in account. Everything that needs the profile
 * goes through this key so one page load makes one request for it.
 */
export const CURRENT_USER_KEY = ["auth", "me"] as const

export function useAuth() {
  const { data: user, isLoading } = useQuery({
    queryKey: CURRENT_USER_KEY,
    queryFn: async () => {
      if (!hasSession()) return null

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
    meta: { silent: true, success: "Welcome back!" },
    mutationFn: (body: { identifier: string; password: string }) =>
      auth.login(body),
    onSuccess: (data) => {
      rememberSession(data)
      queryClient.invalidateQueries({ queryKey: ["auth"] })
      navigate({ to: "/" })
    },
  })
}

export function useRegister() {
  const navigate = useNavigate()

  return useMutation({
    meta: { silent: true, success: "Account created! Please log in." },
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
      const token = refreshToken()
      if (token) {
        await auth.logout(token)
      }
    },
    onSettled: () => {
      forgetSession()
      queryClient.clear()
      navigate({ to: "/login" })
    },
  })
}
