import { describe, expect, it } from "vitest"
import {
  existingIds,
  existingItem,
  keyOf,
  newPendingItem,
  pendingFiles,
} from "@/lib/media"

const file = (name: string) => new File(["x"], name, { type: "image/jpeg" })

describe("media items", () => {
  it("separates what the server holds from what is still local", () => {
    const items = [existingItem("on-server"), newPendingItem(file("new.jpg"))]

    expect(existingIds(items)).toEqual(["on-server"])
    expect(pendingFiles(items).map((each) => each.name)).toEqual(["new.jpg"])
  })

  it("gives every pending item its own key", () => {
    const first = newPendingItem(file("a.jpg"))
    const second = newPendingItem(file("a.jpg"))

    expect(keyOf(first)).not.toBe(keyOf(second))
  })

  it("keys an existing item by its server id", () => {
    expect(keyOf(existingItem("photo-7"))).toBe("photo-7")
  })
})
