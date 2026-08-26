import axios, { AxiosError } from "axios"
import type { z } from "zod/v4"
import {
  ActivityResponse,
  ApiErrorResponse,
  AuthTokenResponse,
  BulkActionResponse,
  CalendarDayResponse,
  EntryResponse,
  ImportResultResponse,
  MediaIdResponse,
  MoodStatsResponse,
  ReminderResponse,
  UserResponse,
  VapidKeyResponse,
  ProviderConnectionResponse,
  DailyMetricResponse,
  RejectionResponse,
  RestoreOutcomeResponse,
  CycleViewResponse,
  PreferencesResponse,
  ApiTokenResponse,
  MintedApiTokenResponse,
  CorrelationRowResponse,
  DimensionPayload,
} from "./schema"
import type {
  ChangePasswordRequest,
  CreateActivityRequest,
  CreateEntryRequest,
  DateSpanParams,
  MetricPayload,
  SubsonicCredential,
  CreateReminderRequest,
  DateRangeParams,
  ListEntriesParams,
  LoginRequest,
  RegisterRequest,
  RenameActivityRequest,
  ReplaceActivityRequest,
  SetCategoryRequest,
  UpdateEntryRequest,
  UpdateProfileRequest,
  UpdateReminderRequest,
} from "./schema"

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL ?? "",
})

api.interceptors.request.use((config) => {
  const token = localStorage.getItem("accessToken")
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

let refreshPromise: Promise<boolean> | null = null

async function tryRefreshToken(): Promise<boolean> {
  const refreshToken = localStorage.getItem("refreshToken")
  if (!refreshToken) return false

  try {
    const { data } = await axios.post(
      `${api.defaults.baseURL}/api/v1/auth/refresh`,
      { refreshToken }
    )
    const parsed = AuthTokenResponse.parse(data)
    localStorage.setItem("accessToken", parsed.accessToken)
    localStorage.setItem("refreshToken", parsed.refreshToken)
    return true
  } catch {
    return false
  }
}

api.interceptors.response.use(undefined, async (error: AxiosError) => {
  const original = error.config
  if (error.response?.status !== 401 || !original) throw error

  if (!refreshPromise) {
    refreshPromise = tryRefreshToken().finally(() => {
      refreshPromise = null
    })
  }

  const refreshed = await refreshPromise
  if (!refreshed) {
    localStorage.removeItem("accessToken")
    localStorage.removeItem("refreshToken")
    window.location.href = "/login"
    return Promise.reject(error)
  }

  return api(original)
})

class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string
  ) {
    super(message)
    this.name = "ApiError"
  }
}

function handleError(err: unknown): never {
  if (err instanceof AxiosError && err.response) {
    const parsed = ApiErrorResponse.safeParse(err.response.data)
    throw new ApiError(
      err.response.status,
      parsed.success ? parsed.data.error.code : "UNKNOWN",
      parsed.success
        ? parsed.data.error.message
        : `Request failed: ${err.response.status}`
    )
  }
  throw err
}

async function request<T>(
  method: "get" | "post" | "patch" | "delete",
  path: string,
  schema: z.ZodType<T>,
  data?: unknown,
  params?: Record<string, string | number | undefined>
): Promise<T> {
  try {
    const res = await api.request({ method, url: path, data, params })
    return schema.parse(res.data)
  } catch (err) {
    return handleError(err)
  }
}

async function requestVoid(
  method: "post" | "put" | "patch" | "delete",
  path: string,
  data?: unknown
): Promise<void> {
  try {
    await api.request({ method, url: path, data })
  } catch (err) {
    handleError(err)
  }
}

async function uploadForm<T>(
  path: string,
  file: File,
  schema: z.ZodType<T>
): Promise<T> {
  const formData = new FormData()
  formData.append("file", file)
  try {
    const res = await api.post(path, formData)
    return schema.parse(res.data)
  } catch (err) {
    return handleError(err)
  }
}

// ── Auth ────────────────────────────────────────────────────────

export const auth = {
  login: (body: LoginRequest) =>
    request("post", "/api/v1/auth/login", AuthTokenResponse, body),

  register: (body: RegisterRequest) =>
    request("post", "/api/v1/users/register", UserResponse, body),

  refresh: (refreshToken: string) =>
    request("post", "/api/v1/auth/refresh", AuthTokenResponse, {
      refreshToken,
    }),

  logout: (refreshToken: string) =>
    requestVoid("post", "/api/v1/auth/logout", { refreshToken }),
}

// ── Providers ───────────────────────────────────────────────────

export const providers = {
  list: () =>
    request("get", "/api/v1/providers", ProviderConnectionResponse.array()),

  connect: (provider: string, credential: SubsonicCredential) =>
    requestVoid("put", `/api/v1/providers/${provider}`, { credential }),

  disconnect: (provider: string) =>
    requestVoid("delete", `/api/v1/providers/${provider}`),

  nowPlaying: () =>
    request(
      "get",
      "/api/v1/providers/now-playing",
      DimensionPayload.nullable()
    ),
}

// ── Cycle ───────────────────────────────────────────────────────

export const cycle = {
  read: () => request("get", "/api/v1/cycle", CycleViewResponse),

  record: (date: string) => requestVoid("put", `/api/v1/cycle/${date}`),

  forget: (date: string) => requestVoid("delete", `/api/v1/cycle/${date}`),

  preferences: () =>
    request("get", "/api/v1/users/me/preferences", PreferencesResponse),

  setTracking: (tracksCycle: boolean) =>
    request("patch", "/api/v1/users/me/preferences", PreferencesResponse, {
      tracksCycle,
    }),
}

// ── API tokens ──────────────────────────────────────────────────

export const tokens = {
  list: () => request("get", "/api/v1/tokens", ApiTokenResponse.array()),

  mint: (name: string) =>
    request("post", "/api/v1/tokens", MintedApiTokenResponse, { name }),

  revoke: (id: string) => requestVoid("delete", `/api/v1/tokens/${id}`),
}

// ── Correlations ────────────────────────────────────────────────

export const correlations = {
  list: (params: DateSpanParams) =>
    request(
      "get",
      "/api/v1/correlations",
      CorrelationRowResponse.array(),
      undefined,
      params
    ),
}

// ── Daily metrics ───────────────────────────────────────────────

export const metrics = {
  list: (params: DateSpanParams) =>
    request(
      "get",
      "/api/v1/metrics",
      DailyMetricResponse.array(),
      undefined,
      params
    ),

  set: (date: string, values: MetricPayload[]) =>
    requestVoid("put", `/api/v1/metrics/${date}`, { metrics: values }),

  rejections: () =>
    request("get", "/api/v1/metrics/rejections", RejectionResponse.array()),
}

// ── Entries ─────────────────────────────────────────────────────

export const entries = {
  list: (params?: ListEntriesParams) =>
    request("get", "/api/v1/entries", EntryResponse.array(), undefined, params),

  get: (id: string) => request("get", `/api/v1/entries/${id}`, EntryResponse),

  create: (body: CreateEntryRequest) =>
    request("post", "/api/v1/entries", EntryResponse, body),

  update: (id: string, body: UpdateEntryRequest) =>
    request("patch", `/api/v1/entries/${id}`, EntryResponse, body),

  delete: (id: string) => requestVoid("delete", `/api/v1/entries/${id}`),

  deleteByDateRange: (params: DateRangeParams) =>
    request(
      "delete",
      "/api/v1/entries/bulk/delete",
      BulkActionResponse,
      undefined,
      params
    ),

  replaceActivity: (body: ReplaceActivityRequest) =>
    request(
      "post",
      "/api/v1/entries/bulk/replace-activity",
      BulkActionResponse,
      body
    ),

  calendar: (params: DateRangeParams) =>
    request(
      "get",
      "/api/v1/entries/calendar",
      CalendarDayResponse.array(),
      undefined,
      params
    ),

  stats: (params?: DateRangeParams) =>
    request(
      "get",
      "/api/v1/entries/stats",
      MoodStatsResponse,
      undefined,
      params
    ),

  filterByMood: (mood: number) =>
    request(
      "get",
      `/api/v1/entries/filter/mood/${mood}`,
      EntryResponse.array()
    ),

  filterByActivity: (activityId: string) =>
    request(
      "get",
      `/api/v1/entries/filter/activity/${activityId}`,
      EntryResponse.array()
    ),
}

// ── Activities ──────────────────────────────────────────────────

export const activities = {
  list: () => request("get", "/api/v1/activities", ActivityResponse.array()),

  get: (id: string) =>
    request("get", `/api/v1/activities/${id}`, ActivityResponse),

  create: (body: CreateActivityRequest) =>
    request("post", "/api/v1/activities", ActivityResponse, body),

  delete: (id: string) => requestVoid("delete", `/api/v1/activities/${id}`),

  rename: (id: string, body: RenameActivityRequest) =>
    requestVoid("patch", `/api/v1/activities/${id}/name`, body),

  setCategory: (id: string, body: SetCategoryRequest) =>
    requestVoid("patch", `/api/v1/activities/${id}/category`, body),

  archive: (id: string) =>
    requestVoid("post", `/api/v1/activities/${id}/archive`),

  unarchive: (id: string) =>
    requestVoid("post", `/api/v1/activities/${id}/unarchive`),
}

// ── Reminders ───────────────────────────────────────────────────

export const reminders = {
  list: () => request("get", "/api/v1/reminders", ReminderResponse.array()),

  get: (id: string) =>
    request("get", `/api/v1/reminders/${id}`, ReminderResponse),

  create: (body: CreateReminderRequest) =>
    request("post", "/api/v1/reminders", ReminderResponse, body),

  update: (id: string, body: UpdateReminderRequest) =>
    request("patch", `/api/v1/reminders/${id}`, ReminderResponse, body),

  delete: (id: string) => requestVoid("delete", `/api/v1/reminders/${id}`),
}

// ── Users ───────────────────────────────────────────────────────

export const users = {
  me: () => request("get", "/api/v1/users/me", UserResponse),

  updateProfile: (body: UpdateProfileRequest) =>
    request("patch", "/api/v1/users/me", UserResponse, body),

  changePassword: (body: ChangePasswordRequest) =>
    requestVoid("patch", "/api/v1/users/me/password", body),

  deleteAccount: () => requestVoid("delete", "/api/v1/users/me"),

  clearData: () => requestVoid("delete", "/api/v1/users/me/data"),
}

// ── Media ───────────────────────────────────────────────────────

export const media = {
  uploadPhoto: (file: File) =>
    uploadForm("/api/v1/media/photos", file, MediaIdResponse),

  uploadVoiceMemo: (file: File) =>
    uploadForm("/api/v1/media/voice-memos", file, MediaIdResponse),

  deletePhoto: (id: string) =>
    requestVoid("delete", `/api/v1/media/photos/${id}`),

  deleteVoiceMemo: (id: string) =>
    requestVoid("delete", `/api/v1/media/voice-memos/${id}`),

  photoUrl: (id: string) => `${api.defaults.baseURL}/api/v1/media/photos/${id}`,
  voiceMemoUrl: (id: string) =>
    `${api.defaults.baseURL}/api/v1/media/voice-memos/${id}`,
}

// ── Data ────────────────────────────────────────────────────────

export const data = {
  completeBackup: () => download("/api/v1/data/backup"),

  shareableExtract: () => download("/api/v1/data/extract"),

  restore: (file: File) =>
    uploadForm("/api/v1/data/restore", file, RestoreOutcomeResponse),

  import: (file: File) =>
    uploadForm("/api/v1/data/import", file, ImportResultResponse),
}

async function download(path: string): Promise<Blob> {
  try {
    const res = await api.get(path, { responseType: "blob" })
    return res.data as Blob
  } catch (err) {
    return handleError(err)
  }
}

// ── Push ───────────────────────────────────────────────────────

export const push = {
  vapidKey: () => request("get", "/api/v1/push/vapid-key", VapidKeyResponse),

  subscribe: (body: { endpoint: string; p256dh: string; auth: string }) =>
    requestVoid("post", "/api/v1/push/subscribe", body),

  unsubscribe: (endpoint: string) =>
    requestVoid("post", "/api/v1/push/unsubscribe", { endpoint }),

  test: () => requestVoid("post", "/api/v1/push/test"),
}

export { ApiError }
