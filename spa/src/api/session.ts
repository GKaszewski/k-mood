const ACCESS_TOKEN_KEY = "accessToken"
const REFRESH_TOKEN_KEY = "refreshToken"

export interface SessionTokens {
  accessToken: string
  refreshToken: string
}

function read(key: string): string | null {
  try {
    return localStorage.getItem(key)
  } catch {
    return null
  }
}

export function accessToken(): string | null {
  return read(ACCESS_TOKEN_KEY)
}

export function refreshToken(): string | null {
  return read(REFRESH_TOKEN_KEY)
}

export function hasSession(): boolean {
  return accessToken() !== null
}

export function rememberSession(tokens: SessionTokens): void {
  try {
    localStorage.setItem(ACCESS_TOKEN_KEY, tokens.accessToken)
    localStorage.setItem(REFRESH_TOKEN_KEY, tokens.refreshToken)
  } catch {
    // A browser refusing storage leaves the session in memory only.
  }
}

export function forgetSession(): void {
  try {
    localStorage.removeItem(ACCESS_TOKEN_KEY)
    localStorage.removeItem(REFRESH_TOKEN_KEY)
  } catch {
    // Nothing to forget if storage is unavailable.
  }
}
