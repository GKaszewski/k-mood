import { useState, useEffect } from "react"
import { useMutation, useQuery } from "@tanstack/react-query"
import { push } from "@/api/client"

function urlBase64ToUint8Array(base64String: string): Uint8Array {
  const padding = "=".repeat((4 - (base64String.length % 4)) % 4)
  const base64 = (base64String + padding).replace(/-/g, "+").replace(/_/g, "/")
  const raw = atob(base64)
  const array = new Uint8Array(raw.length)
  for (let i = 0; i < raw.length; i++) {
    array[i] = raw.charCodeAt(i)
  }
  return array
}

export function usePushNotifications() {
  const [isSubscribed, setIsSubscribed] = useState(false)
  const [isSupported, setIsSupported] = useState(false)

  const vapidKey = useQuery({
    queryKey: ["vapid-key"],
    queryFn: () => push.vapidKey(),
    retry: false,
  })

  useEffect(() => {
    const supported =
      "serviceWorker" in navigator &&
      "PushManager" in window &&
      "Notification" in window
    setIsSupported(supported)

    if (supported) {
      navigator.serviceWorker.ready.then((reg) => {
        reg.pushManager.getSubscription().then((sub) => {
          setIsSubscribed(sub !== null)
        })
      })
    }
  }, [])

  const subscribe = useMutation({
    mutationFn: async () => {
      if (!vapidKey.data) throw new Error("VAPID key not loaded")

      const permission = await Notification.requestPermission()
      if (permission !== "granted") throw new Error("Permission denied")

      const reg = await navigator.serviceWorker.ready
      const keyBytes = urlBase64ToUint8Array(vapidKey.data.publicKey)
      const sub = await reg.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey: keyBytes.buffer as ArrayBuffer,
      })

      const key = sub.getKey("p256dh")
      const auth = sub.getKey("auth")
      if (!key || !auth) throw new Error("Missing push subscription keys")

      await push.subscribe({
        endpoint: sub.endpoint,
        p256dh: btoa(String.fromCharCode(...new Uint8Array(key))),
        auth: btoa(String.fromCharCode(...new Uint8Array(auth))),
      })
    },
    onSuccess: () => setIsSubscribed(true),
    meta: { success: "Notifications enabled" },
  })

  const unsubscribe = useMutation({
    mutationFn: async () => {
      const reg = await navigator.serviceWorker.ready
      const sub = await reg.pushManager.getSubscription()
      if (sub) {
        await push.unsubscribe(sub.endpoint)
        await sub.unsubscribe()
      }
    },
    onSuccess: () => setIsSubscribed(false),
    meta: { success: "Notifications disabled" },
  })

  const testPush = useMutation({
    mutationFn: () => push.test(),
    meta: { success: "Test notification sent" },
  })

  return {
    isSupported,
    isSubscribed,
    isLoading: subscribe.isPending || unsubscribe.isPending,
    isTesting: testPush.isPending,
    hasVapidKey: vapidKey.isSuccess,
    toggle: () => {
      if (isSubscribed) {
        unsubscribe.mutate()
      } else {
        subscribe.mutate()
      }
    },
    sendTest: () => testPush.mutate(),
  }
}
