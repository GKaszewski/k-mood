const CHOSEN_SERVER = "serverUrl"

const BUILT_IN = import.meta.env.VITE_API_URL ?? ""

export function serverUrl(): string {
  try {
    return localStorage.getItem(CHOSEN_SERVER) ?? BUILT_IN
  } catch {
    return BUILT_IN
  }
}

export function isServerChosen(): boolean {
  try {
    return localStorage.getItem(CHOSEN_SERVER) !== null
  } catch {
    return false
  }
}

export function tidyServerUrl(typed: string): string {
  return typed.trim().replace(/\/+$/, "")
}

export function looksLikeAServer(typed: string): boolean {
  const tidied = tidyServerUrl(typed)
  if (!tidied) return false

  try {
    const parsed = new URL(tidied)

    return parsed.protocol === "http:" || parsed.protocol === "https:"
  } catch {
    return false
  }
}

export async function reachable(typed: string): Promise<boolean> {
  try {
    const response = await fetch(`${tidyServerUrl(typed)}/api/v1/server`)

    return response.ok
  } catch {
    return false
  }
}

export function rememberServer(typed: string) {
  localStorage.setItem(CHOSEN_SERVER, tidyServerUrl(typed))
  window.location.reload()
}

export function forgetServer() {
  localStorage.removeItem(CHOSEN_SERVER)
  window.location.reload()
}
