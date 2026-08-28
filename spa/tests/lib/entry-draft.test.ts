import { describe, expect, it } from "vitest"
import {
  emptyDraft,
  hasExtras,
  isWorthKeeping,
  toCreateRequest,
} from "@/lib/entry-draft"
import type { EntryDraft } from "@/lib/entry-draft"

const NOTHING_UPLOADED = { photoIds: [], voiceMemoIds: [] }

function draftWith(changes: Partial<EntryDraft>): EntryDraft {
  return { ...emptyDraft(), mood: 4, ...changes }
}

describe("entry drafts", () => {
  it("is not worth keeping when nothing has been entered", () => {
    expect(isWorthKeeping(emptyDraft())).toBe(false)
  })

  it("is worth keeping once a mood is picked", () => {
    expect(isWorthKeeping(draftWith({}))).toBe(true)
  })

  it("is worth keeping for a note with no mood", () => {
    const draft = { ...emptyDraft(), content: "a rough morning" }

    expect(isWorthKeeping(draft)).toBe(true)
  })

  it("does not count whitespace as a note", () => {
    expect(hasExtras(draftWith({ content: "   " }))).toBe(false)
  })

  it("does not treat a tagged activity as an extra", () => {
    // Activities live in the always-visible part of the form; counting them
    // sprang the "add more" section open the moment one was tapped.
    expect(hasExtras(draftWith({ activityIds: ["reading"] }))).toBe(false)
  })

  it("still keeps a draft that only has activities on it", () => {
    const draft = { ...emptyDraft(), activityIds: ["reading"] }

    expect(isWorthKeeping(draft)).toBe(true)
  })

  it("refuses to build a request without a mood", () => {
    expect(() => toCreateRequest(emptyDraft(), NOTHING_UPLOADED)).toThrowError(
      /without a mood/
    )
  })

  it("omits dimensions that hold nothing", () => {
    const request = toCreateRequest(draftWith({}), NOTHING_UPLOADED)

    expect(request.dimensions).toEqual([])
    expect(request.mood).toBe(4)
  })

  it("trims the note and keeps it as a content dimension", () => {
    const request = toCreateRequest(
      draftWith({ content: "  tired  " }),
      NOTHING_UPLOADED
    )

    expect(request.dimensions).toContainEqual({
      kind: "content",
      text: "tired",
    })
  })

  it("keeps a song only when both title and artist are given", () => {
    const halfNamed = toCreateRequest(
      draftWith({ song: { title: "Teardrop", artist: "" } }),
      NOTHING_UPLOADED
    )
    expect(halfNamed.dimensions).toEqual([])

    const named = toCreateRequest(
      draftWith({ song: { title: "Teardrop", artist: "Massive Attack" } }),
      NOTHING_UPLOADED
    )
    expect(named.dimensions).toContainEqual({
      kind: "song",
      title: "Teardrop",
      artist: "Massive Attack",
    })
  })

  it("uses the ids the upload returned, not the local media", () => {
    const request = toCreateRequest(draftWith({}), {
      photoIds: ["photo-a", "photo-b"],
      voiceMemoIds: ["memo-a"],
    })

    expect(request.dimensions).toContainEqual({
      kind: "photos",
      ids: ["photo-a", "photo-b"],
    })
    expect(request.dimensions).toContainEqual({
      kind: "voiceMemos",
      ids: ["memo-a"],
    })
  })

  it("carries coordinates through as a location dimension", () => {
    const request = toCreateRequest(
      draftWith({ location: { latitude: 52.23, longitude: 21.01 } }),
      NOTHING_UPLOADED
    )

    expect(request.dimensions).toContainEqual({
      kind: "location",
      latitude: 52.23,
      longitude: 21.01,
    })
  })
})
