# Period Compare — Design

Date: 2026-11-14
Status: Draft, awaiting approval (builder check done)

## Intent

Let a User put two stretches of their history side by side — this October vs last October, 2025 vs 2026, summer vs summer, or any two custom spans — and see **everything** that can be compared, as numbers and charts, with an honest signal of which differences are real and which are noise.

The User is a self-described data person: the goal is depth and completeness, not a single headline.

### The rule this design follows (the User's, not negotiable)

- All computation happens on the server.
- Math lives in the **domain** (pure, no I/O). **Use cases** in the application crate load data and call the domain. The HTTP adapter only exposes endpoints.
- The API is the product. The SPA is one client among possible many; any client must get identical numbers without re-implementing logic.
- The SPA provides UX only, built with shadcn components (custom components composed on shadcn primitives when nothing fits).

## Goals

1. One endpoint that takes two date spans and returns the full comparison package: both periods' summaries (normalized and raw), per-row differences, significance marks, and both bucketed trend lines.
2. Normalization so periods of different length or different logging coverage compare fairly.
3. A Compare page in the SPA: two period pickers with presets, then every section laid out at once (layout "B" — one table per section plus overlaid charts).
4. Move the existing dashboard/stats trend bucketing from the SPA (`spa/src/lib/trend-buckets.ts`) to the server, reusing the same domain bucketing, so there is one definition of a trend point for all clients.

## Non-goals

- The server does **not** understand named periods ("summer", "last month"). It accepts two spans; presets are a client concern.
- No comparison of more than two periods.
- No location comparison (raw coordinates; nothing meaningful to compare yet).
- No moon-phase comparison (every period sees nearly the same moon; it is a control in correlations, not a comparable quantity).
- No caching/materialisation of comparisons; computed on request like correlations.

## Anti-goals (would make this a failure even if it works)

- Raw totals presented as the comparison when periods differ in length or coverage (misleading).
- Differences presented without any indication of whether they could be noise, or a "real" mark computed on too few days.
- Any comparison math living in the SPA.
- Smoothed charts that invent values (use the monotone curve, as fixed in `d7b58ce`).

## Decisions made with the User

| Decision | Choice |
|---|---|
| What is compared | Everything comparable: mood, trend, activities, health metrics, weather, music, cycle |
| Server input | Two date spans only, as validated value objects |
| Normalization | Required. Rates, not raw totals, are the primary figures |
| Page shape | Every section shown at once (not a headline view) |
| Layout | Option B: per-section tables (A, B, Δ, diverging bar, significance) + overlaid trend chart aligned on day 1 |
| UI kit | shadcn first; custom components built on shadcn |
| Presets | Provided by the client; full custom ranges always available |
| Trend bucketing migration | In scope, as the last step |

## Domain model

All new domain code lives under a new module `crates/domain/src/comparison/` (one concern per file, per `CODE_STYLE.md`), plus a small extension to `correlation/` for the shared statistics.

### Input value objects

- **`DateSpan`** (existing, `domain::entry::DateSpan`) is the validated span type — whole days, start ≤ end. Reused, not duplicated.
- **`ComparedSpans`** — a new value object holding `a: DateSpan` and `b: DateSpan`. Constructor validates:
  - each span is no longer than a configured maximum (see Constraints), returning `DomainError::InvalidInput` otherwise.
  - Overlapping spans are **allowed** (e.g. "2026 so far" vs "last 90 days" is a legitimate question). But the tests assume independent samples, so **when spans overlap no row is tested**: every row is `NotTested` and the response carries `spansOverlap: true` so clients can say why. *(My call.)*
- **Clipping to today:** a span whose end is after the User's today is clipped to today (in the User's Timezone) by the use case before summarising. The response reports the effective span so clients can show "Jan 1 – Nov 14 (so far)". Days in the future are never counted.

### Period summary (one per side)

A `PeriodSummary` is computed from one period's day-shaped data. Day boundaries come from the User's Timezone (ADR 0001); a day's mood is `DayMood` (mean of that day's entries), the same as correlations use.

- **Coverage:** `days_in_span` (after clipping), `logged_days` (days with ≥1 entry), `coverage = logged_days / days_in_span`, `entries`, `entries_per_logged_day`.
- **Mood:** mean of DayMoods, plus the sample of DayMoods (needed for the significance test); share of logged days at each Mood (by nearest-mood of DayMood — *builder's call, consistent with heatmap; alternative is share of entries*); longest logging streak within the span; longest streak of days at or above Good (≥4) — *my call, a "good streak"*.
- **Activities:** for every activity that appears in either period: `days_with` (logged days carrying it), `share = days_with / logged_days`, mean DayMood on days with it. Archived activities are included if they appear.
- **Health metrics:** for each of `MetricKind::ALL`: `days_with_data`, mean per day with data, total. Mean is the primary figure.
- **Weather:** share per `Condition` of **days that have weather data** (not of all logged days — weather is observed after the fact, ADR 0012, and can be missing, which would otherwise read as a change in the weather), with `days_with_weather` reported as coverage, mean temperature, mean DayMood per Condition. A day's weather = the weather on that day's entries (if several differ, the builder chooses a rule and records it; suggestion: most frequent condition, mean temperature).
- **Music:** lists only, no `ComparisonRow`s or significance. Ranked by **share of entries that carry a song** (normalized), top N artists and top N songs (N from config, default 10), each with count and share. `onlyInA` / `onlyInB` compare **all** artists of each period but are capped at N, ordered by share. *(My call.)*
- **Cycle:** only when the User tracks cycle (`preferences.tracks_cycle()`): mean DayMood per cycle phase. Phase boundaries reuse whatever `CyclePosition` already exposes; if no phase concept exists, bucket `progress()` into fixed quarters and record that as a named constant.

### Differences and significance

A `ComparisonRow` pairs A and B for one quantity and carries:

- `a`, `b` (normalized values), `absolute_change = b - a`, `relative_change = (b - a) / |a|` (absent when a is 0 or missing). For shares, the change is in percentage points.
- `sample_a`, `sample_b` (days that contributed).
- `significance`: one of `LikelyReal`, `Noise`, `NotEnoughData`, `NotTested`.
  - `NotTested` is for descriptive rows that have no sampling question behind them: coverage, logged days, entries, entries per logged day, streaks, metric totals. They are shown as plain figures with Δ, never labelled noise, and are **not** part of the Benjamini–Hochberg family. *(My call.)*
  - Every row is `NotTested` when the spans overlap (see `ComparedSpans`).

Tests:

- **Means** (mood, metrics, temperature, mood-on-activity-days): two-sample **Welch t / z** on the per-day values. Extract a general `welch_two_sample(a: &[f64], b: &[f64]) -> Option<f64>` from the existing `correlation/welch.rs` so both features share it; the existing binary-split function becomes a caller of it.
- **Shares** (activity share, mood share, weather condition share): **two-proportion z-test**. New file in the shared statistics location.
- Standard score → p-value via the existing `correlation/normal.rs` / `p_value.rs`.
- **Multiple comparisons:** the response has dozens of rows; apply the existing Benjamini–Hochberg `Adjustment` across all tested rows (i.e. excluding `NotTested`) of one comparison using `analysis_config.false_discovery_rate`. A row is `LikelyReal` only if it survives adjustment.
- Below `analysis_config.comparison_minimum_sample_size` (default 10, replacing the shared gate for comparisons; see ADR 0021) on either side → `NotEnoughData`, no test run, but values still returned.

The shared statistics (Welch, proportions, normal, p-value, adjustment) should move from `correlation/` into a neutral module (e.g. `domain::statistics`) so comparison does not depend on correlation. *(Builder's call whether to move or re-export; moving is preferred per DRY/SRP.)*

### Trend buckets

A domain `TrendBucketing` (Strategy) turns a series of `(Date, DayMood)` plus entry counts into bucket points. Two strategies, one shared granularity rule:

- **Granularity** from span length: day ≤ 62 days, week ≤ 366 days, otherwise month. Thresholds are named constants. Both sides of a comparison use the **same** granularity (chosen from the longer span).
- **`SpanStartBucketing`, used by Compare (decided with the User):** buckets are counted from each period's own start: fixed windows of 1, 7 or 30 days (named constants; "month" here means a 30-day window, not a calendar month). A's bucket *n* and B's bucket *n* always cover the same number of days, so the overlay is like for like. Labels are relative ("Week 3", "Days 61–90"). The final bucket may be partial and reports its `days_covered`.
- **`CalendarBucketing`, used by the single-period stats trend (Goal 4):** calendar days, weeks and months, the same as today's `spa/src/lib/trend-buckets.ts`. Weeks start on a `weekStartsOn` day **sent by the client** from its locale (decided with the User), so the existing chart doesn't change for anyone. Defaults to Monday when omitted.
- Each point: `offset` (bucket index from the span start), `start_date`, `days_covered`, `mean_mood` (absent for an empty bucket, so charts show gaps instead of interpolating), `logged_days`, `entries` (the current trend tooltip shows entry counts; the migration must not lose them).


## Application layer

- `crates/application/src/comparison/` with `queries.rs` (`ComparisonQuery { user_id, spans: ComparedSpans, minimum_sample_size, false_discovery_rate, top_music_count }`) and `use_cases/compare_periods.rs`.
- The use case mirrors `get_correlations`: resolve Timezone, clip spans to today, load entries per span (`find_by_date_range` with `instants_of`), compose dimensions with `EntryComposer` (weather, activity, song stores), load metrics with `find_by_span`, load cycle calendar when tracked, then call domain to build the two `PeriodSummary`s and the `Comparison`.
- Loads for A and B may run concurrently.
- `use_cases/get_mood_trend.rs` (Goal 4): one span in, trend points out, via the same domain bucketing.

## API

- `GET /api/v1/comparisons?aFrom=YYYY-MM-DD&aTo=…&bFrom=…&bTo=…`
  - New params type `ComparedSpansParams` in `api-types/src/params/`, converting to `ComparedSpans` with the same error shape as `DateSpanParams::into_span` (ADR 0020, one refusal shape).
  - Auth: `JournalReader`, same as correlations.
  - Response `ComparisonResponse` (camelCase), roughly:
    ```
    {
      a: { span: {from,to,clipped}, coverage: {...} },
      b: { ... },
      trend: { granularity, a: [TrendPoint], b: [TrendPoint] },
      sections: {
        overview:   [ComparisonRow],
        mood:       { rows: [ComparisonRow], distribution: { a: [MoodShare], b: [MoodShare] } },
        activities: [ComparisonRow & { activityId, name, category }],
        metrics:    [ComparisonRow & { kind, unit }],
        weather:    [ComparisonRow & { condition? }],
        music:      { a: {artists, songs}, b: {...}, onlyInA: [...], onlyInB: [...] },
        cycle:      [ComparisonRow & { phase }] | null
      }
    }
    ```
    `ComparisonRow` = `{ key, a, b, absoluteChange, relativeChange, sampleA, sampleB, significance }`. Exact field naming is the builder's; it must be documented via utoipa like the correlations handler.
  - Rows come back in a fixed order (like correlations); sorting by change is a client concern.
- `GET /api/v1/entries/trend?from=…&to=…&weekStartsOn=monday|sunday|…` returning `{ granularity, points: [TrendPoint] }` (Goal 4). Takes a plain `DateSpan`; the comparison's maximum span does **not** apply, so the stats "All" period keeps working for long imported histories. *(My call.)*
- OpenAPI updated; SPA zod schema in `spa/src/api/schema.ts` updated to match.

## SPA

- New route under `spa/src/routes/_auth/` (e.g. `compare.tsx`), linked from the statistics page and dashboard alongside the existing metrics link.
- **Period pickers:** two pickers (A green, B blue — colours consistent throughout) built on shadcn `Popover` + `Calendar` (range mode). Preset chips (shadcn `ToggleGroup` or `Button`s): *Same month last year*, *This year vs last year*, *This season vs last season*, *Previous period* (same length, immediately before). Presets only compute two spans client-side.
- Both spans live in the URL search params so comparisons are bookmarkable/shareable.
- **Sections** (all rendered at once, each a shadcn `Card`):
  1. Overview — days, logged days, coverage, entries per logged day; a shadcn `Alert` when coverage differs sharply or one side is thin.
  2. Mood trend — one Recharts chart with both series overlaid on `offset`, `type="monotone"`, gaps not interpolated (`connectNulls={false}`).
  3. Mood — rows + stacked distribution bars for A and B.
  4. Activities — rows sortable by absolute change (client-side sort of server rows).
  5. Health — the eight metrics.
  6. Weather.
  7. Music — top lists side by side, "new in B" / "gone in B" badges.
  8. Cycle — only when present.
- **Row component** (custom, composed on shadcn `Table` + `Badge`): label, A, B, Δ, small diverging bar, significance badge (`likely real` / `noise` / `not enough data`). A `Tooltip` explains what the badge means and the sample sizes.
- Loading via TanStack Query (`useComparison(spans)`), skeletons with shadcn `Skeleton`, as other stats components do.
- **Trend migration (Goal 4):** `useTrend` calls the new trend endpoint; `lib/trend-buckets.ts` and its client aggregation are deleted; `MoodTrendChart` renders server points.

## Constraints

- Follow `CODE_STYLE.md`: small files, newtypes, no magic numbers, tests in sibling `tests/` dirs, no `unwrap` in production code, narrow port traits (no new god-object ports).
- Domain crate stays dependency-free (only `thiserror`/`serde`).
- Maximum span length: a new `analysis_config` value (suggest 5 years / 1830 days) to bound the work per request.
- An ADR should record: normalization rules (per logged day vs per day-with-data), significance testing + FDR across a comparison, clipping to today, and why trend bucketing lives on the server.
- `CONTEXT.md` gains terms for the new language (e.g. **Comparison**, **PeriodSummary**, **Coverage**, **TrendBucket**), in its existing style.

## Testing

- Domain: unit tests for `ComparedSpans` validation, coverage, each summary section, Welch two-sample and two-proportion tests against known values, FDR applied across rows, `NotEnoughData` threshold, bucketing granularity/offset/empty buckets.
- Application: use-case tests with the existing fakes (`domain::testing`), covering timezone day boundaries, clipping to today, cycle-not-tracked, and an activity present in only one period.
- HTTP: handler test for param validation and response shape.
- SPA: whatever level the existing stats components are tested at.

## Left to the builder

- Exact Rust type and file names within the shapes above.
- Whether shared statistics move to `domain::statistics` or are re-exported (moving preferred).
- Weather-per-day rule when a day's entries disagree.
- Exact preset date math in the SPA.
- Visual polish of the page beyond layout B.

## Build order

1. Shared statistics extraction (no behaviour change; correlation tests stay green).
2. Domain: `ComparedSpans`, summaries, comparison rows, significance, trend bucketing.
3. Application use case + API endpoint + OpenAPI.
4. SPA Compare page.
5. Trend endpoint + migrate `MoodTrendChart`/`useTrend`; delete client bucketing.
