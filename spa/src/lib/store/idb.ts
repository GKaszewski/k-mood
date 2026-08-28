/**
 * A small key-value store over IndexedDB. Drafts and queued entries carry
 * photos and recordings, which localStorage cannot hold.
 */

const DB_NAME = "k-mood"
const DB_VERSION = 1

export const DRAFTS = "drafts"
export const OUTBOX = "outbox"

const STORES = [DRAFTS, OUTBOX] as const

export type StoreName = (typeof STORES)[number]

let connection: Promise<IDBDatabase> | null = null

function open(): Promise<IDBDatabase> {
  if (connection) return connection

  connection = new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION)

    request.onupgradeneeded = () => {
      for (const store of STORES) {
        if (!request.result.objectStoreNames.contains(store)) {
          request.result.createObjectStore(store)
        }
      }
    }

    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })

  connection = connection.catch((error) => {
    connection = null
    throw error
  })

  return connection
}

function run<T>(
  store: StoreName,
  mode: IDBTransactionMode,
  work: (store: IDBObjectStore) => IDBRequest<T>
): Promise<T> {
  return open().then(
    (db) =>
      new Promise<T>((resolve, reject) => {
        const transaction = db.transaction(store, mode)
        const request = work(transaction.objectStore(store))

        request.onsuccess = () => resolve(request.result)
        request.onerror = () => reject(request.error)
      })
  )
}

export function put<T>(store: StoreName, key: string, value: T): Promise<void> {
  return run(store, "readwrite", (objects) => objects.put(value, key)).then(
    () => undefined
  )
}

export function get<T>(store: StoreName, key: string): Promise<T | undefined> {
  return run<T | undefined>(store, "readonly", (objects) => objects.get(key))
}

export function remove(store: StoreName, key: string): Promise<void> {
  return run(store, "readwrite", (objects) => objects.delete(key)).then(
    () => undefined
  )
}

export function all<T>(store: StoreName): Promise<T[]> {
  return run<T[]>(store, "readonly", (objects) => objects.getAll())
}

export function isAvailable(): boolean {
  return typeof indexedDB !== "undefined"
}
