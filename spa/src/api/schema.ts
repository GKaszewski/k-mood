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

export const DimensionPayload = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("content"), text: z.string() }),
  z.object({ kind: z.literal("activities"), ids: z.array(uuid) }),
  z.object({ kind: z.literal("photos"), ids: z.array(uuid) }),
  z.object({ kind: z.literal("voiceMemos"), ids: z.array(uuid) }),
  z.object({
    kind: z.literal("location"),
    latitude: z.number().min(-90).max(90),
    longitude: z.number().min(-180).max(180),
  }),
  z.object({
    kind: z.literal("song"),
    title: z.string(),
    artist: z.string(),
    album: z.string().optional(),
    recordingId: uuid.optional(),
  }),
  z.object({
    kind: z.literal("weather"),
    condition: z.enum([
      "clear",
      "cloudy",
      "fog",
      "drizzle",
      "rain",
      "snow",
      "thunderstorm",
    ]),
    temperature: z.number(),
    observedBy: z.string(),
  }),
])
export type DimensionPayload = z.infer<typeof DimensionPayload>

export const EntryResponse = z.object({
  id: uuid,
  userId: uuid,
  mood: Mood,
  moodLabel: z.string(),
  loggedAt: datetime,
  dimensions: z.array(DimensionPayload),
  photoUrls: z.array(z.string()),
  voiceMemoUrls: z.array(z.string()),
  createdAt: datetime,
  updatedAt: datetime,
})
export type EntryResponse = z.infer<typeof EntryResponse>

export const CalendarDayResponse = z.object({
  date: date,
  entries: z.array(EntryResponse),
  dayMood: z.number().nullable(),
  mood: Mood.nullable(),
  moodLabel: z.string().nullable(),
  cycleDay: z.int().nullable(),
})
export type CalendarDayResponse = z.infer<typeof CalendarDayResponse>

export const MoodFrequencyResponse = z.object({
  mood: Mood,
  moodLabel: z.string(),
  count: z.number(),
})
export type MoodFrequencyResponse = z.infer<typeof MoodFrequencyResponse>

export const MoodStatsResponse = z.object({
  average: z.number().nullable(),
  frequency: z.array(MoodFrequencyResponse),
  currentStreak: z.number(),
  totalEntries: z.number(),
  firstLoggedOn: z.string().nullable(),
  lastLoggedOn: z.string().nullable(),
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

export const RefreshedResponse = z.object({
  accessToken: z.string(),
  refreshToken: z.string(),
  expiresAt: datetime,
})
export type RefreshedResponse = z.infer<typeof RefreshedResponse>

export const SignedInResponse = RefreshedResponse.extend({
  user: UserResponse,
})
export type SignedInResponse = z.infer<typeof SignedInResponse>

export const ErrorResponse = z.object({
  error: z.object({
    code: z.string(),
    message: z.string(),
  }),
})
export type ErrorResponse = z.infer<typeof ErrorResponse>

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
  dimensions: z.array(DimensionPayload).optional(),
})
export type CreateEntryRequest = z.infer<typeof CreateEntryRequest>

export const UpdateEntryRequest = z.object({
  mood: Mood,
  loggedAt: z.string().optional(),
  dimensions: z.array(DimensionPayload).optional(),
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
  mood: z.number().int().optional(),
  activity: z.string().optional(),
  updatedSince: z.string().optional(),
  limit: z.number().int().optional(),
  offset: z.number().int().optional(),
})
export type ListEntriesParams = z.infer<typeof ListEntriesParams>

export const DateRangeParams = z.object({
  from: z.string(),
  to: z.string(),
})
export type DateRangeParams = z.infer<typeof DateRangeParams>

export const ProviderConnectionResponse = z.object({
  provider: z.string(),
  connectedAt: datetime,
})
export type ProviderConnectionResponse = z.infer<
  typeof ProviderConnectionResponse
>

export const MetricKind = z.enum([
  "steps",
  "sleepMinutes",
  "awakeMinutes",
  "restingHeartRate",
  "hrv",
  "exerciseMinutes",
  "screenTimeMinutes",
  "alcoholicDrinks",
])
export type MetricKind = z.infer<typeof MetricKind>

export const DailyMetricResponse = z.object({
  date: date,
  kind: MetricKind,
  value: z.int(),
  provider: z.string().optional(),
})
export type DailyMetricResponse = z.infer<typeof DailyMetricResponse>

export const MetricPayload = z.object({
  kind: MetricKind,
  value: z.int().nullable(),
})
export type MetricPayload = z.infer<typeof MetricPayload>

export const SetDailyMetricsRequest = z.object({
  metrics: z.array(MetricPayload),
})
export type SetDailyMetricsRequest = z.infer<typeof SetDailyMetricsRequest>

export const DateSpanParams = z.object({
  from: date,
  to: date,
})
export type DateSpanParams = z.infer<typeof DateSpanParams>

export const CorrelationInputPayload = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("metric"), metric: MetricKind }),
  z.object({ kind: z.literal("moonPhase") }),
  z.object({ kind: z.literal("cycleProgress") }),
  z.object({ kind: z.literal("temperature") }),
  z.object({
    kind: z.literal("activity"),
    activityId: uuid,
    name: z.string(),
  }),
])
export type CorrelationInputPayload = z.infer<typeof CorrelationInputPayload>

export const StrategyScoreResponse = z.object({
  strategy: z.string(),
  coefficient: z.number(),
  heldUp: z.boolean(),
})
export type StrategyScoreResponse = z.infer<typeof StrategyScoreResponse>

export const AgreementResponse = z.object({
  agreeing: z.int(),
  applicable: z.int(),
})
export type AgreementResponse = z.infer<typeof AgreementResponse>

export const CorrelationRowResponse = z.object({
  input: CorrelationInputPayload,
  sampleSize: z.int(),
  agreement: AgreementResponse,
  scores: z.array(StrategyScoreResponse),
})
export type CorrelationRowResponse = z.infer<typeof CorrelationRowResponse>

// ── Period comparison ───────────────────────────────────────────

export const ComparedSpansParams = z.object({
  aFrom: date,
  aTo: date,
  bFrom: date,
  bTo: date,
})
export type ComparedSpansParams = z.infer<typeof ComparedSpansParams>

export const Significance = z.enum([
  "likelyReal",
  "noise",
  "notEnoughData",
  "notTested",
])
export type Significance = z.infer<typeof Significance>

export const ComparisonRowResponse = z.object({
  key: z.string(),
  a: z.number().nullable(),
  b: z.number().nullable(),
  absoluteChange: z.number().nullable(),
  relativeChange: z.number().nullable(),
  sampleA: z.int(),
  sampleB: z.int(),
  significance: Significance,
})
export type ComparisonRowResponse = z.infer<typeof ComparisonRowResponse>

export const MoodComparisonRowResponse = ComparisonRowResponse.extend({
  mood: Mood.nullable(),
  moodLabel: z.string().nullable(),
})
export type MoodComparisonRowResponse = z.infer<
  typeof MoodComparisonRowResponse
>

export const ActivityComparisonRowResponse = ComparisonRowResponse.extend({
  activityId: z.string(),
  name: z.string(),
  category: z.string().nullable(),
})
export type ActivityComparisonRowResponse = z.infer<
  typeof ActivityComparisonRowResponse
>

export const MetricComparisonRowResponse = ComparisonRowResponse.extend({
  kind: MetricKind,
  unit: z.string(),
})
export type MetricComparisonRowResponse = z.infer<
  typeof MetricComparisonRowResponse
>

export const WeatherComparisonRowResponse = ComparisonRowResponse.extend({
  condition: z.string().nullable(),
})
export type WeatherComparisonRowResponse = z.infer<
  typeof WeatherComparisonRowResponse
>

export const CycleComparisonRowResponse = ComparisonRowResponse.extend({
  quarter: z.int().min(1).max(4),
})
export type CycleComparisonRowResponse = z.infer<
  typeof CycleComparisonRowResponse
>

export const PeriodResponse = z.object({
  from: date,
  to: date,
  clipped: z.boolean(),
  daysInSpan: z.int(),
  loggedDays: z.int(),
})
export type PeriodResponse = z.infer<typeof PeriodResponse>

export const ComparedTrendPointResponse = z.object({
  offset: z.int(),
  start: date,
  daysCovered: z.int(),
  meanMood: z.number().nullable(),
  loggedDays: z.int(),
  entries: z.int(),
})
export type ComparedTrendPointResponse = z.infer<
  typeof ComparedTrendPointResponse
>

export const TrendGranularity = z.enum(["day", "week", "month"])
export type TrendGranularity = z.infer<typeof TrendGranularity>

export const ComparedTrendResponse = z.object({
  granularity: TrendGranularity,
  a: z.array(ComparedTrendPointResponse),
  b: z.array(ComparedTrendPointResponse),
})
export type ComparedTrendResponse = z.infer<typeof ComparedTrendResponse>

export const MoodTrendResponse = z.object({
  granularity: TrendGranularity,
  points: z.array(ComparedTrendPointResponse),
})
export type MoodTrendResponse = z.infer<typeof MoodTrendResponse>

export const WeekStartName = z.enum([
  "monday",
  "tuesday",
  "wednesday",
  "thursday",
  "friday",
  "saturday",
  "sunday",
])
export type WeekStartName = z.infer<typeof WeekStartName>

export const MoodTrendParams = z.object({
  from: date,
  to: date,
  weekStartsOn: WeekStartName,
})
export type MoodTrendParams = z.infer<typeof MoodTrendParams>

export const MoodSectionResponse = z.object({
  rows: z.array(MoodComparisonRowResponse),
  distribution: z.object({
    a: z.array(MoodFrequencyResponse),
    b: z.array(MoodFrequencyResponse),
  }),
})
export type MoodSectionResponse = z.infer<typeof MoodSectionResponse>

export const RankedResponse = z.object({
  name: z.string(),
  count: z.int(),
  share: z.number(),
})
export type RankedResponse = z.infer<typeof RankedResponse>

export const RankedSongResponse = z.object({
  artist: z.string(),
  title: z.string(),
  count: z.int(),
  share: z.number(),
})
export type RankedSongResponse = z.infer<typeof RankedSongResponse>

export const MusicResponse = z.object({
  entriesWithSong: z.int(),
  artists: z.array(RankedResponse),
  songs: z.array(RankedSongResponse),
})
export type MusicResponse = z.infer<typeof MusicResponse>

export const MusicComparisonResponse = z.object({
  a: MusicResponse,
  b: MusicResponse,
  onlyInA: z.array(z.string()),
  onlyInB: z.array(z.string()),
})
export type MusicComparisonResponse = z.infer<typeof MusicComparisonResponse>

export const ComparisonResponse = z.object({
  a: PeriodResponse,
  b: PeriodResponse,
  spansOverlap: z.boolean(),
  trend: ComparedTrendResponse,
  overview: z.array(ComparisonRowResponse),
  mood: MoodSectionResponse,
  activities: z.array(ActivityComparisonRowResponse),
  metrics: z.array(MetricComparisonRowResponse),
  weather: z.array(WeatherComparisonRowResponse),
  music: MusicComparisonResponse,
  cycle: z.array(CycleComparisonRowResponse).nullable(),
})
export type ComparisonResponse = z.infer<typeof ComparisonResponse>

export const TokenScope = z.enum([
  "readJournal",
  "writeJournal",
  "writeMetrics",
  "readProfile",
])
export type TokenScope = z.infer<typeof TokenScope>

export const ApiTokenResponse = z.object({
  id: uuid,
  name: z.string(),
  scopes: z.array(TokenScope),
  createdAt: datetime,
  lastUsedAt: datetime.nullable(),
})
export type ApiTokenResponse = z.infer<typeof ApiTokenResponse>

export const MintedApiTokenResponse = z.object({
  token: ApiTokenResponse,
  secret: z.string(),
})
export type MintedApiTokenResponse = z.infer<typeof MintedApiTokenResponse>

export const MintApiTokenRequest = z.object({
  name: z.string(),
  scopes: z.array(TokenScope),
})
export type MintApiTokenRequest = z.infer<typeof MintApiTokenRequest>

export const RejectionResponse = z.object({
  id: uuid,
  origin: z.enum(["import", "storedRow"]),
  provider: z.string().nullable(),
  date: date.nullable(),
  kind: z.string(),
  value: z.int().nullable(),
  reason: z.string(),
  recordedAt: datetime,
})
export type RejectionResponse = z.infer<typeof RejectionResponse>

export const PreferencesResponse = z.object({
  tracksCycle: z.boolean(),
})
export type PreferencesResponse = z.infer<typeof PreferencesResponse>

export const CyclePositionResponse = z.object({
  day: z.int(),
  progress: z.number(),
})
export type CyclePositionResponse = z.infer<typeof CyclePositionResponse>

export const CycleViewResponse = z.object({
  tracking: z.boolean(),
  starts: z.array(date),
  today: CyclePositionResponse.nullable(),
  usualLength: z.int(),
})
export type CycleViewResponse = z.infer<typeof CycleViewResponse>

export const RestoreOutcomeResponse = z.object({
  entries: z.int(),
  metrics: z.int(),
  cycleStarts: z.int(),
  activities: z.int(),
  reminders: z.int(),
  media: z.int(),
  skipped: z.int(),
  unreadable: z.array(z.string()),
})
export type RestoreOutcomeResponse = z.infer<typeof RestoreOutcomeResponse>

export const SubsonicCredential = z.object({
  url: z.string(),
  username: z.string(),
  password: z.string(),
})
export type SubsonicCredential = z.infer<typeof SubsonicCredential>

export const CreateEntriesRequest = z.object({
  entries: z.array(CreateEntryRequest),
})
export type CreateEntriesRequest = z.infer<typeof CreateEntriesRequest>

export const EntryPageResponse = z.object({
  items: z.array(EntryResponse),
  total: z.int(),
  limit: z.int(),
  offset: z.int(),
  hasMore: z.boolean(),
})
export type EntryPageResponse = z.infer<typeof EntryPageResponse>

export const ServerLimitsResponse = z.object({
  maxBodySize: z.int(),
  maxContentLength: z.int(),
  maxPhotos: z.int(),
  maxVoiceMemos: z.int(),
  maxActivitiesPerEntry: z.int(),
  maxEntriesPerPage: z.int(),
  maxImportDays: z.int(),
})
export type ServerLimitsResponse = z.infer<typeof ServerLimitsResponse>

export const MetricKindResponse = z.object({
  name: z.string(),
  unit: z.string(),
  minimum: z.int(),
  maximum: z.int(),
})
export type MetricKindResponse = z.infer<typeof MetricKindResponse>

export const ServerInfoResponse = z.object({
  version: z.string(),
  registrationOpen: z.boolean(),
  pushEnabled: z.boolean(),
  weatherEnabled: z.boolean(),
  nowPlayingProvider: z.string().nullable(),
  accessTokenTtlSeconds: z.int(),
  limits: ServerLimitsResponse,
  tokenScopes: z.array(z.object({ name: TokenScope, describes: z.string() })),
  metricKinds: z.array(MetricKindResponse),
  moods: z.array(z.object({ value: z.int(), label: z.string() })),
  weatherConditions: z.array(z.string()),
})
export type ServerInfoResponse = z.infer<typeof ServerInfoResponse>
