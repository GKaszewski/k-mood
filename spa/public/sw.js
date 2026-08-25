const CACHE_NAME = "k-mood-v1"
const STATIC_ASSETS = [
  "/",
  "/manifest.json",
  "/logo192.png",
  "/logo512.png",
  "/favicon.ico",
]

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(STATIC_ASSETS))
  )
  self.skipWaiting()
})

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k))
        )
      )
  )
  self.clients.claim()
})

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url)

  if (url.pathname.startsWith("/api/")) return

  if (event.request.mode === "navigate") {
    event.respondWith(
      fetch(event.request)
        .then((response) => {
          const clone = response.clone()
          caches
            .open(CACHE_NAME)
            .then((cache) => cache.put(event.request, clone))
          return response
        })
        .catch(() => caches.match("/"))
    )
    return
  }

  event.respondWith(
    caches.match(event.request).then((cached) => {
      const fetched = fetch(event.request).then((response) => {
        const clone = response.clone()
        caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone))
        return response
      })
      return cached || fetched
    })
  )
})

self.addEventListener("push", (event) => {
  const data = event.data?.json() ?? {}
  const title = data.title || "K-Mood"
  const options = {
    body: data.body || "How are you feeling?",
    icon: "/logo192.png",
    badge: "/logo192.png",
    data: { url: data.url || "/" },
  }
  event.waitUntil(self.registration.showNotification(title, options))
})

self.addEventListener("notificationclick", (event) => {
  event.notification.close()
  const url = event.notification.data?.url || "/"
  event.waitUntil(
    clients.matchAll({ type: "window" }).then((windowClients) => {
      for (const client of windowClients) {
        if (client.url.includes(url) && "focus" in client) return client.focus()
      }
      return clients.openWindow(url)
    })
  )
})
