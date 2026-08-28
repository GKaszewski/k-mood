import { beforeEach, describe, expect, it, vi } from "vitest"
import type * as ApiClient from "@/api/client"
import { ApiError } from "@/api/client"
import { emptyDraft } from "@/lib/entry-draft"
import type { QueuedEntry } from "@/lib/store/outbox"

// Hoisted alongside vi.mock, which runs before the imports above.
const { entries, media, store } = vi.hoisted(() => ({
  entries: { create: vi.fn() },
  media: {
    uploadPhoto: vi.fn(),
    uploadVoiceMemo: vi.fn(),
    deletePhoto: vi.fn(),
    deleteVoiceMemo: vi.fn(),
  },
  store: {
    queued: [] as unknown[],
    dequeued: [] as string[],
    attempts: [] as Array<{ id: string; error: string; counted: boolean }>,
  },
}))

vi.mock("@/api/client", async () => {
  const actual = await vi.importActual<typeof ApiClient>("@/api/client")

  return { ApiError: actual.ApiError, entries, media }
})

vi.mock("@/lib/store/outbox", () => ({
  queuedEntries: async () => store.queued,
  dequeue: async (id: string) => {
    store.dequeued.push(id)
  },
  noteAttempt: async (id: string, error: string, counted: boolean) => {
    store.attempts.push({ id, error, counted })
  },
}))

const { flushOutbox, MAX_AUTOMATIC_ATTEMPTS } =
  await import("@/lib/outbox-sync")

function queued(id: string, changes: Partial<QueuedEntry> = {}): QueuedEntry {
  return {
    id,
    draft: { ...emptyDraft(), mood: 3 },
    queuedAt: `2026-01-0${id}T10:00:00.000Z`,
    attempts: 0,
    ...changes,
  }
}

describe("flushing the outbox", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    store.queued = []
    store.dequeued = []
    store.attempts = []
    entries.create.mockResolvedValue({ id: "server-id" })
  })

  it("sends the oldest entry first", async () => {
    store.queued = [queued("1"), queued("2"), queued("3")]

    await flushOutbox()

    expect(store.dequeued).toEqual(["1", "2", "3"])
  })

  it("reports how many it sent", async () => {
    store.queued = [queued("1"), queued("2")]

    await expect(flushOutbox()).resolves.toEqual({ sent: 2, failed: 0 })
  })

  it("counts a refusal against the entry and records why", async () => {
    store.queued = [queued("1")]
    entries.create.mockRejectedValue(
      new ApiError(422, "UNPROCESSABLE", "That activity is not yours")
    )

    await expect(flushOutbox()).resolves.toEqual({ sent: 0, failed: 1 })
    expect(store.dequeued).toEqual([])
    expect(store.attempts).toEqual([
      { id: "1", error: "That activity is not yours", counted: true },
    ])
  })

  it("does not count a failure that never reached the server", async () => {
    // A connection that is down says nothing about whether the entry is good,
    // so it must not burn through the entry's attempts.
    store.queued = [queued("1")]
    entries.create.mockRejectedValue(new Error("Network Error"))

    await flushOutbox()

    expect(store.attempts).toEqual([
      { id: "1", error: "No connection to the server", counted: false },
    ])
  })

  it("keeps sending the rest after one fails", async () => {
    store.queued = [queued("1"), queued("2")]
    entries.create
      .mockRejectedValueOnce(new ApiError(422, "NOPE", "refused"))
      .mockResolvedValueOnce({ id: "server-id" })

    await expect(flushOutbox()).resolves.toEqual({ sent: 1, failed: 1 })
    expect(store.dequeued).toEqual(["2"])
  })

  it("takes back media it uploaded when the entry never lands", async () => {
    const withPhoto = queued("1")
    withPhoto.draft.photos = [
      { kind: "pending", localId: "local-1", file: new File([""], "a.jpg") },
    ]
    store.queued = [withPhoto]

    media.uploadPhoto.mockResolvedValue({ id: "uploaded-photo" })
    entries.create.mockRejectedValue(new ApiError(500, "BOOM", "server broke"))

    await flushOutbox()

    expect(media.uploadPhoto).toHaveBeenCalledOnce()
    expect(media.deletePhoto).toHaveBeenCalledWith("uploaded-photo")
  })

  it("leaves alone an entry the server has refused too many times", async () => {
    store.queued = [queued("1", { attempts: MAX_AUTOMATIC_ATTEMPTS })]

    await expect(flushOutbox()).resolves.toEqual({ sent: 0, failed: 0 })
    expect(entries.create).not.toHaveBeenCalled()
  })
})
