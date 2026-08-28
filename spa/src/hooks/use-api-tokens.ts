import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { tokens } from "@/api/client"
import type { MintApiTokenRequest } from "@/api/schema"

const TOKENS = ["api-tokens"]

export function useApiTokens() {
  return useQuery({
    queryKey: TOKENS,
    queryFn: () => tokens.list(),
  })
}

export function useMintApiToken() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (body: MintApiTokenRequest) => tokens.mint(body),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: TOKENS }),
  })
}

export function useRevokeApiToken() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) => tokens.revoke(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: TOKENS }),
  })
}
