import { useQuery } from "@tanstack/react-query"
import { server } from "@/api/client"

const SERVER = ["server"]

export function useServerInfo() {
  return useQuery({
    queryKey: SERVER,
    queryFn: () => server.info(),
    staleTime: Infinity,
  })
}
