import { z } from "zod/v4"

// ── Shared primitives ───────────────────────────────────────────

const uuid = z.uuid()
const datetime = z.iso.datetime({ offset: true })
const date = z.iso.date()

export const Mood = z.int().min(1).max(5)
export type Mood = z.infer<typeof Mood>

export const MoodLabel = z.enum(["Awful", "Bad", "Meh", "Good", "Rad"])
export type MoodLabel = z.infer<typeof MoodLabel>

// ── Responses ───────────────────────────────────────────────────

export const ActivityResponse = z.object({
  id: uuid,
  name: z.string(),
  category: z.string().nullable(),
  archived: z.boolean(),
  createdAt: datetime,
})
export type ActivityResponse = z.infer<typeof ActivityResponse>

export const EntryResponse = z.object({
  id: uuid,
  userId: uuid,
  mood: Mood,
  moodLabel: z.string(),
  loggedAt: datetime,
  activities: z.array(uuid),
  content: z.string().nullable(),
  photos: z.array(uuid),
  photoUrls: z.array(z.string()),
  voiceMemos: z.array(uuid),
  voiceMemoUrls: z.array(z.string()),
  createdAt: datetime,
  updatedAt: datetime,
})
export type EntryResponse = z.infer<typeof EntryResponse>

export const CalendarDayResponse = z.object({
  date: date,
  entries: z.array(EntryResponse),
  dominantMood: Mood.nullable(),
  dominantMoodLabel: z.string().nullable(),
})
export type CalendarDayResponse = z.infer<typeof CalendarDayResponse>

export const MoodFrequency = z.object({
  mood: Mood,
  moodLabel: z.string(),
  count: z.number(),
})
export type MoodFrequency = z.infer<typeof MoodFrequency>

export const MoodStatsResponse = z.object({
  average: z.number().nullable(),
  frequency: z.array(MoodFrequency),
  currentStreak: z.number(),
  totalEntries: z.number(),
})
export type MoodStatsResponse = z.infer<typeof MoodStatsResponse>

export const UserResponse = z.object({
  id: uuid,
  username: z.string(),
  email: z.string(),
  displayName: z.string().nullable(),
  timezone: z.string().nullable(),
  role: z.string(),
  createdAt: datetime,
})
export type UserResponse = z.infer<typeof UserResponse>

export const DayScheduleResponse = z.object({
  monday: z.string().nullable(),
  tuesday: z.string().nullable(),
  wednesday: z.string().nullable(),
  thursday: z.string().nullable(),
  friday: z.string().nullable(),
  saturday: z.string().nullable(),
  sunday: z.string().nullable(),
})
export type DayScheduleResponse = z.infer<typeof DayScheduleResponse>

export const ReminderResponse = z.object({
  id: uuid,
  schedule: DayScheduleResponse,
  enabled: z.boolean(),
  createdAt: datetime,
})
export type ReminderResponse = z.infer<typeof ReminderResponse>

export const BulkActionResponse = z.object({
  affectedCount: z.number(),
})
export type BulkActionResponse = z.infer<typeof BulkActionResponse>

export const CorrelationResponse = z.object({
  activityId: uuid,
  correlation: z.number().nullable(),
})
export type CorrelationResponse = z.infer<typeof CorrelationResponse>

export const ImportResultResponse = z.object({
  imported: z.number(),
  skipped: z.number(),
  errors: z.array(z.string()),
})
export type ImportResultResponse = z.infer<typeof ImportResultResponse>

export const MediaIdResponse = z.object({
  id: uuid,
})
export type MediaIdResponse = z.infer<typeof MediaIdResponse>

export const AuthTokenResponse = z.object({
  accessToken: z.string(),
  refreshToken: z.string(),
  expiresAt: datetime,
  user: UserResponse.optional(),
})
export type AuthTokenResponse = z.infer<typeof AuthTokenResponse>

export const ApiErrorResponse = z.object({
  error: z.object({
    code: z.string(),
    message: z.string(),
  }),
})
export type ApiErrorResponse = z.infer<typeof ApiErrorResponse>

export const VapidKeyResponse = z.object({
  publicKey: z.string(),
})
export type VapidKeyResponse = z.infer<typeof VapidKeyResponse>

// ── Requests ────────────────────────────────────────────────────

export const LoginRequest = z.object({
  identifier: z.string().min(1),
  password: z.string().min(1),
})
export type LoginRequest = z.infer<typeof LoginRequest>

export const RegisterRequest = z.object({
  username: z.string().min(1),
  email: z.email(),
  password: z.string().min(1),
})
export type RegisterRequest = z.infer<typeof RegisterRequest>

export const CreateEntryRequest = z.object({
  mood: Mood,
  loggedAt: z.string().optional(),
  activityIds: z.array(z.string()).optional(),
  content: z.string().optional(),
  photoIds: z.array(z.string()).optional(),
  voiceMemoIds: z.array(z.string()).optional(),
})
export type CreateEntryRequest = z.infer<typeof CreateEntryRequest>

export const UpdateEntryRequest = z.object({
  mood: Mood,
  loggedAt: z.string().optional(),
  activityIds: z.array(z.string()).optional(),
  content: z.string().optional(),
  photoIds: z.array(z.string()).optional(),
  voiceMemoIds: z.array(z.string()).optional(),
})
export type UpdateEntryRequest = z.infer<typeof UpdateEntryRequest>

export const CreateActivityRequest = z.object({
  name: z.string().min(1),
  category: z.string().nullable().optional(),
})
export type CreateActivityRequest = z.infer<typeof CreateActivityRequest>

export const RenameActivityRequest = z.object({
  newName: z.string().min(1),
})
export type RenameActivityRequest = z.infer<typeof RenameActivityRequest>

export const SetCategoryRequest = z.object({
  category: z.string().nullable(),
})
export type SetCategoryRequest = z.infer<typeof SetCategoryRequest>

export const ReplaceActivityRequest = z.object({
  oldActivityId: z.string(),
  newActivityId: z.string(),
})
export type ReplaceActivityRequest = z.infer<typeof ReplaceActivityRequest>

export const CreateReminderRequest = z.object({
  monday: z.string().nullable().optional(),
  tuesday: z.string().nullable().optional(),
  wednesday: z.string().nullable().optional(),
  thursday: z.string().nullable().optional(),
  friday: z.string().nullable().optional(),
  saturday: z.string().nullable().optional(),
  sunday: z.string().nullable().optional(),
})
export type CreateReminderRequest = z.infer<typeof CreateReminderRequest>

export const UpdateReminderRequest = z.object({
  schedule: CreateReminderRequest.optional(),
  enabled: z.boolean().optional(),
})
export type UpdateReminderRequest = z.infer<typeof UpdateReminderRequest>

export const UpdateProfileRequest = z.object({
  displayName: z.string().nullable().optional(),
  timezone: z.string().nullable().optional(),
})
export type UpdateProfileRequest = z.infer<typeof UpdateProfileRequest>

export const ChangePasswordRequest = z.object({
  currentPassword: z.string().min(1),
  newPassword: z.string().min(1),
})
export type ChangePasswordRequest = z.infer<typeof ChangePasswordRequest>

// ── Query params ────────────────────────────────────────────────

export const ListEntriesParams = z.object({
  from: z.string().optional(),
  to: z.string().optional(),
  limit: z.number().int().optional(),
  offset: z.number().int().optional(),
})
export type ListEntriesParams = z.infer<typeof ListEntriesParams>

export const DateRangeParams = z.object({
  from: z.string(),
  to: z.string(),
})
export type DateRangeParams = z.infer<typeof DateRangeParams>
