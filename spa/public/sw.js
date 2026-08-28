/*
 * The app shell cache and push handling.
 *
 * A new worker deliberately does NOT take over on its own: assets are served
 * cache-first, so swapping them under a running page can hand it chunks from
 * two different builds. It waits until the page asks, which it does after
 * telling the reader a new version is ready.
 */

const CACHE_NAME = "k-mood-v2"
const APP_SHELL = "/"

const STATIC_ASSETS = [
  APP_SHELL,
  "/manifest.json",
  "/logo192.png",
  "/logo512.png",
  "/favicon.ico",
]

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(STATIC_ASSETS))
  )
})

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key))
        )
      )
      .then(() => self.clients.claim())
  )
})

self.addEventListener("message", (event) => {
  if (event.data?.type === "SKIP_WAITING") self.skipWaiting()
})

function cache(request, response) {
  if (!response.ok) return response

  const copy = response.clone()
  caches
    .open(CACHE_NAME)
    .then((store) => store.put(request, copy))
    .catch(() => {})

  return response
}

self.addEventListener("fetch", (event) => {
  const { request } = event
  const url = new URL(request.url)

  // The journal is never cached, wherever the server lives.
  if (url.pathname.startsWith("/api/")) return
  if (request.method !== "GET") return
  if (url.origin !== self.location.origin) return

  if (request.mode === "navigate") {
    event.respondWith(
      fetch(request)
        .then((response) => cache(request, response))
        .catch(() => caches.match(APP_SHELL))
    )
    return
  }

  event.respondWith(
    caches.match(request).then((cached) => {
      const fresh = fetch(request)
        .then((response) => cache(request, response))
        .catch(() => cached)

      return cached ?? fresh
    })
  )
})

self.addEventListener("push", (event) => {
  const data = event.data?.json() ?? {}

  event.waitUntil(
    self.registration.showNotification(data.title || "K-Mood", {
      body: data.body || "How are you feeling?",
      icon: "/logo192.png",
      badge: "/logo192.png",
      data: { url: data.url || APP_SHELL },
    })
  )
})

self.addEventListener("notificationclick", (event) => {
  event.notification.close()
  const target = event.notification.data?.url || APP_SHELL

  event.waitUntil(
    self.clients
      .matchAll({ type: "window", includeUncontrolled: true })
      .then((windows) => {
        for (const client of windows) {
          if (client.url.includes(target) && "focus" in client) {
            return client.focus()
          }
        }
        return self.clients.openWindow(target)
      })
  )
})
