import { beforeEach, describe, expect, it } from "vitest"
import {
  accessToken,
  forgetSession,
  hasSession,
  refreshToken,
  rememberSession,
} from "@/api/session"

const TOKENS = { accessToken: "access-1", refreshToken: "refresh-1" }

describe("session", () => {
  beforeEach(() => localStorage.clear())

  it("has no session before anything is remembered", () => {
    expect(hasSession()).toBe(false)
    expect(accessToken()).toBeNull()
  })

  it("remembers and reads back both tokens", () => {
    rememberSession(TOKENS)

    expect(hasSession()).toBe(true)
    expect(accessToken()).toBe("access-1")
    expect(refreshToken()).toBe("refresh-1")
  })

  it("forgets every key it wrote", () => {
    rememberSession(TOKENS)
    forgetSession()

    expect(hasSession()).toBe(false)
    expect(accessToken()).toBeNull()
    expect(refreshToken()).toBeNull()
    // The bug this guards: deleting an account cleared snake_case keys that
    // were never written, leaving a live token behind.
    expect(localStorage.length).toBe(0)
  })
})
