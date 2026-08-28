import { ApiError } from "@/api/client"

const UNREACHABLE =
  "Could not reach the server. Check your connection, or the server address."

/**
 * What to tell the reader about a failed request. A refusal carries the
 * server's own words; anything else never got there, and saying "invalid
 * credentials" when the server is down sends people to reset a working
 * password.
 */
export function messageFor(error: unknown): string {
  if (error instanceof ApiError) return error.message
  return UNREACHABLE
}

export function wasRefused(error: unknown): boolean {
  return error instanceof ApiError
}
