import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { providers } from "@/api/client"
import type { SubsonicCredential } from "@/api/schema"

const CONNECTIONS = ["provider-connections"]

export function useProviderConnections() {
  return useQuery({
    queryKey: CONNECTIONS,
    queryFn: () => providers.list(),
  })
}

export function useConnectProvider(provider: string) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (credential: SubsonicCredential) =>
      providers.connect(provider, credential),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: CONNECTIONS }),
  })
}

export function useDisconnectProvider(provider: string) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: () => providers.disconnect(provider),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: CONNECTIONS }),
  })
}

export function useNowPlaying() {
  return useMutation({
    mutationFn: () => providers.nowPlaying(),
  })
}
