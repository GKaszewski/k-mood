# k-mood

A personal mood tracking journal. Users log how they feel throughout the day, tag entries with activities, and review trends over time. Multi-user, self-hosted, single server with multiple clients.

## Language

### Core

**MoodEntry**:
A single mood record — the aggregate root. Carries exactly one Mood, the instant it was logged, and nothing else. Belongs to exactly one User. Everything else attaches as EntryDimensions. Multiple MoodEntries per Date are allowed.
_Avoid_: Log, journal entry, record, mood log

**EntryDimension**:
An optional aspect of a MoodEntry, stored independently of it — Content, Activities, photos, voice memos, weather, location, song. Each kind is self-contained: it owns its own type, its own storage, and its own validation, and knows nothing about the others. A MoodEntry is complete without any of them.
_Avoid_: Facet, attribute, extra, metadata, attachment

**Mood**:
One of five discrete states representing how the user feels, mapped to a 1–5 ordinal scale: Awful (1), Bad (2), Meh (3), Good (4), Rad (5). The ordering is a domain truth — Rad is better than Good. Required on every MoodEntry.
_Avoid_: Feeling, emotion, state, score

**Content**:
Optional markdown text attached to a MoodEntry. Serves both quick annotations ("went for a walk") and longer journal-style writing. No domain-level length limit — the application layer enforces configurable maximums.
_Avoid_: Note, quick note, description, journal entry

**Activity**:
A named item from a user's personal catalog, tagged onto MoodEntries. Covers heterogeneous concepts (social contexts like "friends", health actions like "exercise", sleep indicators like "good sleep") under one umbrella term. Each Activity belongs to exactly one User. Activities are archivable — archived Activities remain on historical entries but cannot be tagged onto new ones.
_Avoid_: Tag, label, habit, tracker

**Category**:
A display-only grouping label stored as an optional string on an Activity (e.g., "social", "health", "sleep"). The domain stores it but never interprets it — no domain logic references categories. Exists purely for UI organization across multiple clients.
_Avoid_: Group, section, type

### People

**User**:
A registered account identified by username and email. Has a role (Admin or User) and a Timezone. Owns their own Activity catalog, MoodEntries, and Reminders.
_Avoid_: Account, member, profile

**Timezone**:
The IANA zone a User lives their days in. It is what turns an instant into a Date, so anything day-shaped — the calendar, a streak, a DailyMetric — is unanswerable without it. Clients set it from the platform; a User who somehow has none is told to set one rather than being given a silently wrong answer.
_Avoid_: TZ, offset, locale, region

### Scheduling

**Reminder**:
A per-user notification schedule. Each Reminder defines an `Option<Time>` for each day of the week — `Some(20:00)` means remind at 8 PM, `None` means skip that day. Can be enabled or disabled. A User can have multiple Reminders (e.g., one for morning check-in, one for evening). The domain defines when to remind; clients decide the message and delivery mechanism.
_Avoid_: Notification, alert, alarm, push

### Measurement

**DailyMetric**:
A single measured or self-reported value for one User on one Date — a MetricKind plus its value. Not an aggregate; uniqueness of (User, Date, MetricKind) is a structural guarantee, not an invariant. Independent of MoodEntry: a Date may carry DailyMetrics with no MoodEntries and vice versa.
_Avoid_: Health data, daily record, day log, stat, data point

**Source**:
Where a DailyMetric's value came from — either Manual, meaning the User stated it, or a Provider, meaning something else did. A DailyMetric sourced Manual is never overwritten by an import; a manual write always supersedes a Provider's.
_Avoid_: Origin, author, integration

**Provider**:
An external system that supplies data. Identified only by name — the domain knows a Provider exists and what it claimed, never how it is reached, authenticated, or parsed. Which Providers exist is a deployment concern, not a domain one.
_Avoid_: Integration, service, source, connector

**ApiToken**:
A named, revocable, non-expiring credential a User mints for an automation that cannot hold a session. Scoped to writing DailyMetrics and nothing else — it cannot read entries or touch the account. Its name is the Provider its writes are attributed to, so two tokens of one account cannot share a name. The value is shown once at mint time and held only as a digest.
_Avoid_: API key, secret, personal access token, integration key

**ProviderConnection**:
A User's standing authorization to a Provider, held as an opaque credential the domain never inspects. Each User supplies their own, and holds at most one per Provider — connecting again replaces it. The domain knows only that a User is or is not connected to a named Provider.
_Avoid_: Account link, integration, credentials, connection

**MetricKind**:
The closed set of things a DailyMetric can measure: steps, sleep minutes, awake minutes, resting heart rate, HRV, exercise minutes, screen time minutes, alcoholic drinks. Each MetricKind owns its unit and its valid range — the unit is never carried alongside the value. Membership is earned: a MetricKind must have real day-to-day variance, be measured well enough to trust, and not restate one already present. Extending the set is the only way to support a new measurement.
_Avoid_: Metric type, field, measurement name

**Date**:
The calendar day a DailyMetric belongs to, resolved by converting an instant into the User's Timezone. The single day boundary in the system — MoodEntries group onto the same Date for calendar and analytics.
_Avoid_: Day, logged date, local date

**DateSpan**:
A run of consecutive Dates, inclusive of both ends. What day-keyed data is read by, and distinct from the range of instants that MoodEntries are deleted by — a span of days has no time of day, so nothing about it depends on an offset.
_Avoid_: Date range, period, window, interval

**RejectedMetric**:
A reading that could not be used, kept where the account holder can read it — what arrived, from which Provider, on which Date, and why it was refused. Receives both readings that arrive broken from an importer and stored rows a later build can no longer read. Nothing in it was stored as a DailyMetric; it is a record of absence, not of data. Only the most recent are kept.
_Avoid_: Error log, failed import, invalid metric, audit trail

**CompleteBackup**:
Everything an account knows, written so that nothing is lost if it has to be restored — every MoodEntry with every EntryDimension, every DailyMetric, every CycleStart, the Activity catalogue, Reminders, UserPreferences and all media. Restores through its own path, never through the importer that reads foreign formats. Carries no credentials and no RejectedMetrics.
_Avoid_: Export, dump, archive, snapshot

**ShareableExtract**:
A readable document of mood, Content and Activities, for handing to another person. Carries nothing else — no Location, no Song, no DailyMetric, no CycleStart, no media — and cannot be restored from. Distinct from a CompleteBackup by intent, not by configuration: a single artifact with options would produce files that look like backups and are not.
_Avoid_: Partial export, filtered export, share link

### Ambient

**Weather**:
The conditions at the place and instant a MoodEntry was logged — a Condition and a temperature in celsius — expressed in one canonical vocabulary so that any two MoodEntries' Weather are comparable. Always attributed to a Provider, never to the User: Weather is observed, not stated, so a User editing an entry cannot remove it and absence from an edit is not a request to delete it. Resolved after the entry is saved, never during it.
_Avoid_: Forecast, conditions, temperature

**Condition**:
The closed vocabulary Weather is expressed in — clear, cloudy, fog, drizzle, rain, snow, thunderstorm. The domain's words, not a Provider's: every Provider's own codes are mapped onto these at the boundary, because correlating across inconsistent labels means nothing. Deliberately coarse; a Provider distinguishing light from heavy rain reports rain.
_Avoid_: Weather code, description, icon, summary

**Location**:
The coordinates a MoodEntry was logged at. Supplied by the client, optional, and the prerequisite for Weather.
_Avoid_: GPS, position, place, geo

**CycleStart**:
A Date on which a User's menstrual period began. Recorded once per cycle, never per day. Cycle day for any Date is derived from the most recent preceding CycleStart and is never stored — correcting a CycleStart therefore corrects every Date that depends on it. Recorded only while the User has cycle tracking on; turning it off hides every derived cycle day without forgetting what was recorded.
_Avoid_: Period, cycle day, menstrual day

**CyclePosition**:
Where a Date sits in its cycle — the cycle day, counting from one, and the progress through the cycle from 0 at its start to 1 at its end. Both are derived. Progress is measured against the next CycleStart where one exists and against the median observed length where the cycle is still running, so a late cycle sits at its end rather than beyond it. A Date more than ninety days after the last CycleStart has no position: that is a missed record, not a long cycle.
_Avoid_: Cycle phase, day number, cycle stage

**UserPreferences**:
The optional features a User has turned on. Separate from User, which carries identity and credentials — a preference is not a fact about who someone is. Absent until something is turned on, and every preference is off when absent.
_Avoid_: Settings, options, config, flags

**Song**:
What a User was listening to when a MoodEntry was logged, held as its own title, artist, and album rather than as a pointer elsewhere — a MoodEntry stays readable with nothing else reachable. Carries an external recording identity when one is known, which is what makes two MoodEntries about the same recording comparable. Captured at the moment of logging only; a MoodEntry never acquires a Song later.
_Avoid_: Track, music, now playing, recording

**MoonPhase**:
The lunar phase on a given Date. Derived from the Date on read and never stored — it is a function of the calendar, not an observation, so there is nothing to persist and no value that can go stale.
_Avoid_: Lunar phase, moon

### Background work

**Job**:
A record that some background work is wanted, carrying its kind, the thing it is about, its status, how many times it has been attempted and why the last attempt failed. Never the source of truth: no Job may be enqueued unless a query over stored data can independently rediscover the same work, so losing one costs promptness and never data. A Job that has used every attempt stops being retried and stays visible instead of vanishing.
_Avoid_: Task, message, event, queue item

**Sweep**:
The query that rediscovers work from stored data — "songs with no recording identity" — and enqueues whatever it finds. What makes the queue safe to lose. Every Job kind owes one, and work that cannot be expressed as such a query does not belong on the queue.
_Avoid_: Scan, reconciler, cron, catch-up

### Analysis

**DayMood**:
The mean of every MoodEntry mood on one Date. The single mood value used wherever a Date needs one — analysis and calendar alike. A Date with no MoodEntries has no DayMood.
_Avoid_: Dominant mood, average mood, daily mood, overall mood

**CorrelationInput**:
Anything a CorrelationStrategy can score against DayMood — every MetricKind, every Activity, plus MoonPhase, which is not a MetricKind because it is never stored. Each input's series has one shape: a MetricKind and MoonPhase are continuous, an Activity is presence, and a Strategy scores one shape, so an incompatible pairing cannot be constructed. Extending the set is deliberate: each input added costs an expected false positive across the whole result set.
_Avoid_: Variable, factor, feature, predictor

**Agreement**:
How many of the Strategies that can score an input point the same way, out of how many apply. The headline for a result, in place of any single coefficient. An Activity has one applicable Strategy, so its agreement is one of one — which is a statement about the measurement, not a claim of corroboration.
_Avoid_: Consensus, confidence, score, robustness

**CorrelationStrategy**:
A named method for scoring the relationship between a MetricKind and DayMood — Pearson, Spearman, Kendall, mean difference. Several are computed over the same data and every one returns the same shape, so they are directly comparable. Agreement among Strategies is the signal; no single Strategy's value is authoritative.
_Avoid_: Algorithm, correlation method, analyzer, engine

**Adjustment**:
A correction applied across a whole set of CorrelationStrategy results to account for how many were tested at once. Operates on a set, never on a single result, and is therefore never itself a CorrelationStrategy. It marks results; it never hides them. Benjamini-Hochberg, controlling false discovery rate, at a configured threshold. A marked result held up once the number of comparisons in its Family was accounted for — a second axis of robustness beside Agreement, never a verdict.
_Avoid_: Correction, p-value adjustment, filter, significance

**Family**:
The set of results one Adjustment is computed over. A Family is a question, not a screen: MetricKinds form one, Activities another, because how many Activities a User keeps has nothing to do with whether their sleep tracks their mood. Adjustment is computed per Family within a single CorrelationStrategy.
_Avoid_: Group, test set, batch, comparison set
