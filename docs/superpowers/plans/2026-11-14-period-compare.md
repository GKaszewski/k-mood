# Period Compare Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Compare two date spans across every comparable dimension on the server, expose it as one endpoint, render it in a Compare page, and move trend bucketing from the SPA to the server.

**Architecture:** Pure maths in `domain` (`statistics`, `comparison`, `trend`), a use case per endpoint in `application` mirroring `get_correlations`, thin axum handlers + api-types mappers, and a TanStack Router page in the SPA composed from shadcn components.

**Tech Stack:** Rust (domain/application/axum/utoipa), React + TanStack Router/Query + Recharts + zod + shadcn/ui, bun.

**Spec:** `docs/superpowers/specs/2026-11-14-period-compare-design.md`

## Global Constraints

- Follow `CODE_STYLE.md`: one concern per file (~100–150 lines max), newtypes, no magic numbers (named constants), no `unwrap`/`expect` in production code, `thiserror` in domain/application.
- Tests never in production files: domain tests in `crates/domain/tests/<module>/<name>_test.rs` registered via `#[path]` in `crates/domain/tests/<module>.rs`; same layout in `crates/application/tests/`.
- Domain crate gains no dependencies.
- Port traits stay narrow; no new god-object ports. No new ports are needed by this plan.
- Monotone curves only in charts (`type="monotone"`); empty buckets are gaps (`connectNulls={false}`).
- No comparison or bucketing maths in the SPA. The SPA may only compute preset date spans and sort server rows.
- `make check` passes at the end of every task (fmt, clippy `-D warnings`, `cargo test --workspace`, SPA check/lint/typecheck).
- Commits: conventional prefix (`feat:`, `refactor:`, `docs:`), no co-author trailer.

## Review Focus

1. **Day boundaries in a non-UTC timezone** — an entry at 00:30 Warsaw time belongs to that Warsaw date in both periods. Pinned in Task 6 (`entries_near_midnight_count_on_the_users_day`).
2. **Period B still running (ends after today)** — clipped to today; `days_in_span` counts elapsed days only; response `clipped: true`. Pinned in Task 2 and Task 6.
3. **A period with zero entries** — every mean is absent, relative change absent, rows `NotEnoughData`, no panic/NaN in JSON. Pinned in Task 5 (`an_empty_period_yields_absent_values_not_nan`).
4. **An activity/artist present in only one period** — row still appears with 0 share on the other side. Pinned in Task 4 and Task 6.
5. **Malformed or reversed query params / span over the maximum** — 400/422 in the one refusal shape (ADR 0020). Pinned in Task 7.

## Decision raised during planning (confirm at review)

`analysis_config.minimum_sample_size` defaults to **30**, which would mark nearly every month-vs-month row `NotEnoughData`. This plan adds a separate `analysis.comparison_minimum_sample_size` (default **10**) used only by Compare. Correlations keep 30.

---

### Task 1: Extract shared statistics into `domain::statistics`

**Files:**
- Create: `crates/domain/src/statistics/mod.rs`, `normal.rs`, `p_value.rs`, `welch.rs`, `two_proportion.rs`, `false_discovery.rs`
- Modify: `crates/domain/src/lib.rs` (add `pub mod statistics;`), `crates/domain/src/correlation/mod.rs`, `correlation/welch.rs`, `correlation/adjustment.rs`, `correlation/correlation_strategy.rs`; delete `correlation/normal.rs`, `correlation/p_value.rs`
- Test: `crates/domain/tests/statistics.rs`, `crates/domain/tests/statistics/{welch_test,two_proportion_test,false_discovery_test}.rs`

**Interfaces:**
- Produces:
  - `domain::statistics::PValue` (moved unchanged; `domain::correlation::PValue` stays as a `pub use` re-export so callers don't break)
  - `domain::statistics::welch_two_sample(a: &[f64], b: &[f64]) -> Option<f64>` — standard score of `mean(b) - mean(a)`; `None` when either side has < 2 values or the pooled spread is 0
  - `domain::statistics::two_proportion(hits_a: usize, total_a: usize, hits_b: usize, total_b: usize) -> Option<f64>` — pooled two-proportion z of `p_b - p_a`; `None` when a total is 0 or the pooled proportion is 0 or 1
  - `domain::statistics::FalseDiscoveryControl::at(rate: f64)` with `.holds_up(&self, p_values: &[PValue]) -> Vec<bool>` (Benjamini–Hochberg; the body of today's `Adjustment::holds_up` moves here, and `Adjustment::holds_up` delegates to it)

- [ ] **Step 1: Write failing tests**

```rust
// welch_test.rs
#[test] fn welch_scores_a_higher_second_sample_as_positive() {
    let z = welch_two_sample(&[1.0, 2.0, 3.0, 4.0], &[3.0, 4.0, 5.0, 6.0]).unwrap();
    assert!((z - 2.1909).abs() < 1e-3);
}
#[test] fn welch_needs_two_values_on_each_side() {
    assert_eq!(welch_two_sample(&[1.0], &[2.0, 3.0]), None);
}
#[test] fn welch_has_nothing_to_say_about_two_constant_samples() {
    assert_eq!(welch_two_sample(&[3.0, 3.0], &[3.0, 3.0]), None);
}
// two_proportion_test.rs
#[test] fn two_proportion_matches_a_textbook_value() {
    // 18/100 vs 31/100, pooled 0.245 → z ≈ 2.1373
    assert!((two_proportion(18, 100, 31, 100).unwrap() - 2.1373).abs() < 1e-3);
}
#[test] fn two_proportion_refuses_an_empty_side() { assert_eq!(two_proportion(0, 0, 3, 10), None); }
#[test] fn two_proportion_refuses_when_everyone_or_no_one_did_it() {
    assert_eq!(two_proportion(10, 10, 5, 5), None);
}
// false_discovery_test.rs — copy the existing plain-holds_up cases from correlation/adjustment_test.rs, calling FalseDiscoveryControl::at(0.10).holds_up(..)
```

- [ ] **Step 2:** `cargo test -p domain --test statistics` → FAIL (module missing).
- [ ] **Step 3:** Implement. `correlation::welch::welch_standard_score` keeps its signature and becomes: split by presence, then `welch_two_sample(&absent, &present)` (order preserves today's sign: present minus absent).
- [ ] **Step 4:** `cargo test -p domain` → all PASS, including the unchanged `tests/correlation/*`.
- [ ] **Step 5:** Commit `refactor: move shared statistics into domain::statistics`.

---

### Task 2: `ComparedSpans` and span clipping

**Files:**
- Create: `crates/domain/src/comparison/mod.rs`, `compared_spans.rs`, `maximum_span.rs`
- Modify: `crates/domain/src/entry/date_span.rs` (add methods), `crates/domain/src/lib.rs`
- Test: `crates/domain/tests/comparison.rs`, `crates/domain/tests/comparison/compared_spans_test.rs`, `crates/domain/tests/entry/date_span_test.rs` (create if absent, register in `tests/entry.rs`)

**Interfaces:**
- Produces:
  - `DateSpan::days(&self) -> i64` (inclusive count), `DateSpan::overlaps(&self, other: &DateSpan) -> bool`, `DateSpan::clipped_to(&self, today: Date) -> Option<ClippedSpan>` (`None` when the span starts after today)
  - `domain::entry::ClippedSpan { span: DateSpan, clipped: bool }` (new file `entry/clipped_span.rs`, getters `span()`, `was_clipped()`)
  - `domain::comparison::MaximumSpan(i64)` newtype, `MaximumSpan::of_days(days: i64) -> Self`, `days()`
  - `domain::comparison::ComparedSpans::new(a: DateSpan, b: DateSpan, maximum: MaximumSpan) -> Result<Self, DomainError>`; getters `a()`, `b()`, `overlap(&self) -> bool`

- [ ] **Step 1: Failing tests**

```rust
#[test] fn a_span_of_one_day_has_one_day() { assert_eq!(span("2026-10-01", "2026-10-01").days(), 1); }
#[test] fn october_has_thirty_one_days() { assert_eq!(span("2026-10-01", "2026-10-31").days(), 31); }
#[test] fn a_running_year_is_clipped_to_today() {
    let clipped = span("2026-01-01", "2026-12-31").clipped_to(date("2026-11-14")).unwrap();
    assert_eq!(clipped.span().days(), 318);
    assert!(clipped.was_clipped());
}
#[test] fn a_finished_span_is_not_clipped() { assert!(!span("2025-10-01","2025-10-31").clipped_to(date("2026-11-14")).unwrap().was_clipped()); }
#[test] fn a_span_entirely_in_the_future_has_nothing_to_compare() { assert!(span("2027-01-01","2027-01-31").clipped_to(date("2026-11-14")).is_none()); }
#[test] fn touching_spans_overlap_on_the_shared_day() { assert!(span("2026-01-01","2026-01-10").overlaps(&span("2026-01-10","2026-01-20"))); }
#[test] fn compared_spans_refuse_a_span_longer_than_the_maximum() {
    let result = ComparedSpans::new(span("2020-01-01","2026-01-01"), span("2026-01-01","2026-01-31"), MaximumSpan::of_days(1830));
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
#[test] fn compared_spans_allow_overlap_and_say_so() {
    let spans = ComparedSpans::new(span("2026-01-01","2026-06-30"), span("2026-06-01","2026-08-31"), MaximumSpan::of_days(1830)).unwrap();
    assert!(spans.overlap());
}
```

- [ ] **Step 2:** `cargo test -p domain --test comparison --test entry` → FAIL.
- [ ] **Step 3:** Implement. Overlap is computed on the requested spans (before clipping).
- [ ] **Step 4:** Tests PASS.
- [ ] **Step 5:** Commit `feat: compared spans value object`.

---

### Task 3: Trend bucketing strategies

**Files:**
- Create: `crates/domain/src/trend/mod.rs`, `granularity.rs`, `trend_day.rs`, `trend_point.rs`, `trend_bucketing.rs`, `span_start_bucketing.rs`, `calendar_bucketing.rs`, `week_start.rs`
- Modify: `crates/domain/src/lib.rs`
- Test: `crates/domain/tests/trend.rs`, `crates/domain/tests/trend/{granularity_test,span_start_test,calendar_test}.rs`

**Interfaces:**
- Produces:
  - `enum Granularity { Day, Week, Month }`, `Granularity::for_days(days: i64) -> Self` (constants `LONGEST_DAILY = 62`, `LONGEST_WEEKLY = 366`)
  - `struct TrendDay { date: Date, mood: DayMood, entries: usize }` (constructor `new`)
  - `struct TrendPoint { offset: usize, start: Date, days_covered: usize, mean_mood: Option<f64>, logged_days: usize, entries: usize }` (public getters)
  - `trait TrendBucketing { fn bucket(&self, span: &DateSpan, days: &[TrendDay], granularity: Granularity) -> Vec<TrendPoint>; }`
  - `struct SpanStartBucketing;` — windows of `DAY_WINDOW = 1`, `WEEK_WINDOW = 7`, `MONTH_WINDOW = 30` days from `span.start()`; last window may be partial
  - `struct CalendarBucketing { week_start: WeekStart }`; `enum WeekStart { Monday, Tuesday, …, Sunday }` with `Default = Monday` and `from_name(&str) -> Option<Self>` (lowercase English names)
  - `mean_mood` = mean of `DayMood::value()` over logged days in the bucket.

- [ ] **Step 1: Failing tests**

```rust
#[test] fn a_month_is_drawn_in_days() { assert_eq!(Granularity::for_days(31), Granularity::Day); }
#[test] fn sixty_three_days_are_drawn_in_weeks() { assert_eq!(Granularity::for_days(63), Granularity::Week); }
#[test] fn a_leap_year_is_drawn_in_weeks() { assert_eq!(Granularity::for_days(366), Granularity::Week); }
#[test] fn two_years_are_drawn_in_months() { assert_eq!(Granularity::for_days(730), Granularity::Month); }

#[test] fn span_start_weeks_count_from_the_first_day_whatever_weekday_it_is() {
    // span Wed 2026-10-07..2026-10-20, Week granularity → 2 points starting 10-07 and 10-14, 7 days each
}
#[test] fn span_start_last_window_may_be_partial() {
    // 2026-10-01..2026-10-10, Week → second point days_covered == 3
}
#[test] fn a_bucket_with_no_logged_days_is_a_gap_not_a_zero() {
    // point.mean_mood() == None, logged_days() == 0
}
#[test] fn calendar_weeks_start_on_the_requested_day() {
    // span Wed 2026-10-07..2026-10-20, WeekStart::Sunday → first point start 2026-10-04, offsets 0,1,2
}
#[test] fn calendar_months_are_whole_months() {
    // span 2025-01-15..2025-03-10 Month → starts 2025-01-01, 2025-02-01, 2025-03-01
}
#[test] fn entries_are_summed_per_bucket() { /* two days, 2 and 3 entries, one bucket → entries() == 5 */ }
```

- [ ] **Step 2:** `cargo test -p domain --test trend` → FAIL.
- [ ] **Step 3:** Implement. Calendar `days_covered` counts only days inside the span (so the first and last buckets can be partial).
- [ ] **Step 4:** PASS.
- [ ] **Step 5:** Commit `feat: trend bucketing strategies`.

---

### Task 4: Period summary

**Files:**
- Create in `crates/domain/src/comparison/`: `period_day.rs`, `period_input.rs`, `period_summary.rs`, `coverage.rs`, `streaks.rs`, `day_weather.rs`, `cycle_quarter.rs`, `music_summary.rs`
- Test: `crates/domain/tests/comparison/{period_summary_test,streaks_test,day_weather_test,music_summary_test}.rs`

**Interfaces:**
- Consumes: `ClippedSpan` (Task 2), `DayMood`, `ActivityId`, `Weather`/`Condition`/`Celsius`, `Song`/`ArtistName`, `MetricKind`, `CyclePosition`.
- Produces:
  - `struct PeriodDay { date, mood: DayMood, entries: usize, activities: BTreeSet<ActivityId>, weather: Vec<Weather>, songs: Vec<Song>, cycle: Option<CyclePosition> }` — one per logged day, built by the application layer
  - `struct PeriodInput { span: ClippedSpan, days: Vec<PeriodDay>, metrics: BTreeMap<MetricKind, BTreeMap<Date, f64>>, tracks_cycle: bool }`
  - `struct PeriodSummary` built by `PeriodSummary::of(input: &PeriodInput) -> Self`, exposing:
    - `coverage() -> Coverage { days_in_span, logged_days, entries }` with `ratio() -> Option<f64>` and `entries_per_logged_day() -> Option<f64>`
    - `day_moods() -> Vec<f64>`, `mood_counts() -> BTreeMap<Mood, usize>` (by `DayMood::rounded()`, every Mood present, zeros included)
    - `longest_logging_streak() -> usize`, `longest_good_streak() -> usize` (consecutive dates with `DayMood::value() >= GOOD_THRESHOLD` where `GOOD_THRESHOLD = Mood::Good as f64`)
    - `activity_days(&ActivityId) -> usize`, `moods_with_activity(&ActivityId) -> Vec<f64>`, `activities() -> BTreeSet<ActivityId>`
    - `metric_values(MetricKind) -> Vec<f64>` (per day with data)
    - `days_with_weather() -> usize`, `condition_days(Condition) -> usize`, `temperatures() -> Vec<f64>`, `moods_in(Condition) -> Vec<f64>`
    - `moods_in_quarter(CycleQuarter) -> Vec<f64>`, `tracks_cycle() -> bool`
    - `music() -> &MusicSummary`
  - `DayWeather::of(&[Weather]) -> Option<(Condition, f64)>` — most frequent condition, ties broken by `Condition::ALL` order; mean temperature
  - `enum CycleQuarter { First, Second, Third, Fourth }`, `CycleQuarter::of(progress: f64) -> Self` (boundaries 0.25 / 0.5 / 0.75 as named constants)
  - `struct MusicSummary { entries_with_song: usize, artists: Vec<Ranked<ArtistName>>, songs: Vec<Ranked<(ArtistName, SongTitle)>> }`; `Ranked<T> { item: T, count: usize, share: f64 }`; `MusicSummary::of(days, top: usize)` keeps **all** artists internally (`all_artists() -> BTreeSet<ArtistName>`) and exposes top N, ordered by share desc then name

- [ ] **Step 1: Failing tests** (helpers build `PeriodDay`s for 2026-10-01.. with given moods)

```rust
#[test] fn coverage_counts_logged_days_against_the_days_in_the_span() { /* 31-day span, 27 logged → ratio 27/31 */ }
#[test] fn an_empty_period_has_no_ratio_of_entries_per_day() { /* 0 logged → entries_per_logged_day() None, ratio Some(0.0) */ }
#[test] fn mood_counts_include_moods_never_felt() { /* all Meh → Awful count 0 present */ }
#[test] fn a_gap_breaks_a_logging_streak() { /* 1,2,3, 5,6 → 3 */ }
#[test] fn a_good_streak_needs_good_or_better_days() { /* 4,5,3,4,4,4 → 3 */ }
#[test] fn mood_with_an_activity_only_counts_days_carrying_it() {}
#[test] fn metrics_are_counted_on_days_with_data_even_without_entries() {}
#[test] fn a_day_of_mixed_weather_takes_the_most_frequent_condition() {}
#[test] fn cycle_progress_of_point_six_is_the_third_quarter() { assert_eq!(CycleQuarter::of(0.6), CycleQuarter::Third); }
#[test] fn music_ranks_by_share_of_entries_with_a_song() {}
#[test] fn music_keeps_only_the_top_n_but_remembers_every_artist() {}
```

- [ ] **Step 2:** `cargo test -p domain --test comparison` → FAIL.
- [ ] **Step 3:** Implement. Split `PeriodSummary` accessors across the small files listed if it exceeds ~150 lines.
- [ ] **Step 4:** PASS.
- [ ] **Step 5:** Commit `feat: period summary`.

---

### Task 5: Comparison rows and significance

**Files:**
- Create in `crates/domain/src/comparison/`: `quantity.rs`, `measure.rs`, `significance.rs`, `comparison_row.rs`, `comparison.rs`, `comparison_rules.rs`, `music_comparison.rs`
- Test: `crates/domain/tests/comparison/{measure_test,comparison_test}.rs`

**Interfaces:**
- Consumes: `PeriodSummary` (Task 4), `welch_two_sample`, `two_proportion`, `PValue::from_standard_score`, `FalseDiscoveryControl` (Task 1), `ComparedSpans` (Task 2).
- Produces:
  - `enum Quantity { DaysInSpan, LoggedDays, Coverage, Entries, EntriesPerLoggedDay, AverageMood, LongestLoggingStreak, LongestGoodStreak, MoodShare(Mood), ActivityShare(ActivityId), MoodWithActivity(ActivityId), MetricMean(MetricKind), MetricDaysWithData(MetricKind), MetricTotal(MetricKind), WeatherShare(Condition), MeanTemperature, MoodInWeather(Condition), MoodInCycleQuarter(CycleQuarter) }`
  - `enum Measure { Figure { a: Option<f64>, b: Option<f64> }, Mean { a: Vec<f64>, b: Vec<f64> }, Share { hits_a: usize, total_a: usize, hits_b: usize, total_b: usize } }` with `values() -> (Option<f64>, Option<f64>)`, `samples() -> (usize, usize)`, `standard_score() -> Option<f64>` (`Figure` → `None`)
  - `enum Significance { LikelyReal, Noise, NotEnoughData, NotTested }`
  - `struct ComparisonRow { quantity, a: Option<f64>, b: Option<f64>, absolute_change: Option<f64>, relative_change: Option<f64>, sample_a: usize, sample_b: usize, significance }` — `relative_change` is `None` when `a` is `None`, `b` is `None`, or `a == 0.0`. Shares are fractions (0–1); the absolute change of a share is therefore in fraction points (client renders ×100 as "pp").
  - `struct ComparisonRules { minimum_sample_size: usize, false_discovery_rate: f64 }`
  - `struct Comparison { a: PeriodSummary, b: PeriodSummary, overlap: bool, rows: Vec<ComparisonRow>, music: MusicComparison, trend: (Vec<TrendPoint>, Vec<TrendPoint>), granularity: Granularity }`
  - `Comparison::between(a: PeriodSummary, b: PeriodSummary, spans: &ComparedSpans, trend_a: &[TrendDay], trend_b: &[TrendDay], rules: ComparisonRules) -> Self` — granularity from the longer clipped span, `SpanStartBucketing`
  - `MusicComparison { a: MusicSummary, b: MusicSummary, only_in_a: Vec<ArtistName>, only_in_b: Vec<ArtistName> }` — set difference of `all_artists()`, capped at top N by share
- Row construction rules (fixed order = the enum order above; activities in `ActivityId` order; metrics in `MetricKind::ALL`; conditions in `Condition::ALL`; cycle rows only when both sides `tracks_cycle()`):
  - `Figure` for DaysInSpan, LoggedDays, Coverage, Entries, EntriesPerLoggedDay, both streaks, MetricDaysWithData, MetricTotal
  - `Mean` for AverageMood, MoodWithActivity, MetricMean, MeanTemperature, MoodInWeather, MoodInCycleQuarter
  - `Share` for MoodShare (total = logged days), ActivityShare (total = logged days), WeatherShare (total = days with weather)
  - Significance: `Figure` → `NotTested`; if `overlap` → `NotTested`; if either sample < `minimum_sample_size` → `NotEnoughData`; if `standard_score()` is `None` → `NotEnoughData`; else collect p-values and run `FalseDiscoveryControl` once across all tested rows → `LikelyReal` / `Noise`.

- [ ] **Step 1: Failing tests**

```rust
#[test] fn descriptive_rows_are_not_tested() { /* Coverage row significance == NotTested */ }
#[test] fn overlapping_spans_test_nothing() { /* every row NotTested, comparison.overlap == true */ }
#[test] fn a_side_below_the_minimum_has_not_enough_data() {}
#[test] fn a_clear_rise_in_mood_is_likely_real() { /* A: 30 days of 2s and 3s, B: 30 days of 4s and 5s */ }
#[test] fn a_trivial_wobble_is_noise() {}
#[test] fn the_correction_runs_across_every_tested_row() {
    // many rows each with p just under 0.10 → after BH some are Noise
}
#[test] fn an_empty_period_yields_absent_values_not_nan() {
    // A has no logged days: every Mean row a == None, relative_change == None, no f64::is_nan anywhere
}
#[test] fn relative_change_is_absent_when_a_was_zero() {}
#[test] fn an_activity_seen_only_in_b_still_gets_a_row_with_zero_share_in_a() {}
#[test] fn cycle_rows_appear_only_when_the_cycle_is_tracked() {}
#[test] fn artists_only_in_one_period_are_listed() {}
#[test] fn both_trends_share_the_granularity_of_the_longer_span() {}
```

- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** PASS.
- [ ] **Step 5:** Commit `feat: period comparison with significance`.

---

### Task 6: `compare_periods` use case + config

**Files:**
- Create: `crates/application/src/comparison/{mod.rs,queries.rs,use_cases/mod.rs,use_cases/compare_periods.rs,period_days.rs}`
- Modify: `crates/application/src/lib.rs`, `crates/config/src/analysis_config.rs`, `config.example.toml`
- Test: `crates/application/tests/comparison.rs`, `crates/application/tests/comparison/compare_periods_test.rs`

**Interfaces:**
- Consumes: Tasks 2–5; `timezone_for`, `today_in`, `date_of`, `day_moods`, `instants_of` from `application::day`; `EntryComposer`; `calendar_of`; `preferences_of`.
- Produces:
  - `AnalysisConfig` gains `comparison_minimum_sample_size: usize` (default `10`), `maximum_comparison_days: i64` (default `1830`), `top_music_count: usize` (default `10`) — named `DEFAULT_*` constants, added to `config.example.toml` under `[analysis]`
  - `ComparisonQuery { user_id: UserId, a: DateSpan, b: DateSpan, maximum_span: MaximumSpan, rules: ComparisonRules, top_music_count: usize }`
  - `compare_periods::Deps { entries: Arc<dyn MoodEntryQueryPort>, metrics: Arc<dyn DailyMetricQueryPort>, activities: Arc<dyn ActivityQueryPort>, cycles: Arc<dyn CycleStartQueryPort>, dimensions: Vec<Arc<dyn EntryDimensionPort>>, preferences: Arc<dyn UserPreferencesQueryPort>, users: Arc<dyn UserQueryPort> }`
  - `compare_periods::execute(query, &deps) -> Result<PeriodComparison, ApplicationError>`
  - `PeriodComparison { comparison: Comparison, spans: (ClippedSpan, ClippedSpan), activities: BTreeMap<ActivityId, Activity> }` (activities for labels, from `find_by_user` so archived ones resolve)
- Flow: build `ComparedSpans` (maps `DomainError` like other use cases) → timezone → clip each span to `today_in`; a span wholly in the future → `ApplicationError` validation refusal → load + compose + metrics for A and B concurrently (`tokio::try_join!` only if `application` already depends on tokio; otherwise `futures::try_join!`; otherwise sequential) → `period_days.rs` turns composed entries into `Vec<PeriodDay>` and `Vec<TrendDay>` → `PeriodSummary::of` ×2 → `Comparison::between`.

- [ ] **Step 1: Failing tests** (style of `tests/correlation/get_correlations_test.rs`: `InMemoryStore`, `test_user`, Warsaw timezone)

```rust
#[tokio::test] async fn a_better_october_is_compared_against_last_october() { /* average mood row a≈2.5 b≈4.5 */ }
#[tokio::test] async fn entries_near_midnight_count_on_the_users_day() { /* entry 2026-10-31T23:30+01:00 belongs to B (Oct), not Nov */ }
#[tokio::test] async fn a_running_period_is_clipped_to_today() { /* B ends in the future → spans.1.was_clipped() */ }
#[tokio::test] async fn a_period_wholly_in_the_future_is_refused() {}
#[tokio::test] async fn a_span_beyond_the_maximum_is_refused() {}
#[tokio::test] async fn an_archived_activity_still_has_a_label() {}
#[tokio::test] async fn metrics_are_compared_per_day_with_data() {}
#[tokio::test] async fn cycle_is_skipped_when_not_tracked() {}
```

- [ ] **Step 2:** `cargo test -p application --test comparison` → FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** PASS; `cargo test --workspace` PASS.
- [ ] **Step 5:** Commit `feat: compare periods use case`.

---

### Task 7: `GET /api/v1/comparisons`

**Files:**
- Create: `crates/api-types/src/params/comparison.rs`, `crates/api-types/src/responses/comparison.rs`, `crates/api-types/src/mappers/comparison.rs`, `crates/adapters/http-axum/src/handlers/comparisons.rs`
- Modify: the three `mod.rs` files in api-types, `handlers/mod.rs`, `router.rs` (nest `/comparisons`), `openapi.rs`
- Test: `crates/adapters/http-axum/tests/openapi_contract_test.rs` (existing contract tests must pass), `crates/api-types` mapper tests if the crate has a tests dir (else add `crates/api-types/tests/comparison_test.rs`)

**Interfaces:**
- Consumes: Task 6.
- Produces:
  - `ComparedSpansParams { a_from: String, a_to: String, b_from: String, b_to: String }` (camelCase → `aFrom`, …) with `into_spans(self) -> Result<(DateSpan, DateSpan), ApiValidationError>` reusing `parse_date`
  - Response types (all `#[serde(rename_all = "camelCase")]`, `utoipa::ToSchema`):
    - `ComparisonResponse { a: PeriodResponse, b: PeriodResponse, spansOverlap: bool, trend: ComparedTrendResponse, overview: Vec<ComparisonRowResponse>, mood: MoodSectionResponse, activities: Vec<ActivityComparisonRowResponse>, metrics: Vec<MetricComparisonRowResponse>, weather: Vec<WeatherComparisonRowResponse>, music: MusicComparisonResponse, cycle: Option<Vec<CycleComparisonRowResponse>> }`
    - `PeriodResponse { from, to, clipped, daysInSpan, loggedDays }`
    - `ComparisonRowResponse { key: String, a: Option<f64>, b: Option<f64>, absoluteChange, relativeChange, sampleA, sampleB, significance: SignificanceResponse }` — `key` values: `daysInSpan`, `loggedDays`, `coverage`, `entries`, `entriesPerLoggedDay`, `averageMood`, `longestLoggingStreak`, `longestGoodStreak`, `moodShare`, `activityShare`, `moodWithActivity`, `metricMean`, `metricDaysWithData`, `metricTotal`, `weatherShare`, `meanTemperature`, `moodInWeather`, `moodInCycleQuarter`
    - `SignificanceResponse` = `"likelyReal" | "noise" | "notEnoughData" | "notTested"`
    - Section row types flatten `ComparisonRowResponse` and add: activity `{ activityId, name, category }`; metric `{ kind, unit }` (from `MetricKind::name()/unit()`); weather `{ condition: Option<String> }` (None for `meanTemperature`); mood section `{ rows, distribution: { a: Vec<MoodFrequencyResponse>, b: … } }` reusing the existing `MoodFrequencyResponse`; cycle `{ quarter: 1..=4 }`
    - `ComparedTrendResponse { granularity: "day"|"week"|"month", a: Vec<TrendPointResponse>, b: Vec<TrendPointResponse> }`; `TrendPointResponse { offset, start, daysCovered, meanMood: Option<f64>, loggedDays, entries }`
    - `MusicComparisonResponse { a: MusicResponse, b: MusicResponse, onlyInA: Vec<String>, onlyInB: Vec<String> }`; `MusicResponse { entriesWithSong, artists: Vec<RankedResponse>, songs: Vec<RankedSongResponse> }`
  - Handler `comparisons::handle_compare` — `JournalReader`, `Params<ComparedSpansParams>`, builds `Deps` from `AppState` (`dimensions: state.dimensions.clone()`), utoipa doc in the style of `handlers/correlations.rs` describing normalization, overlap, and `notTested`.

- [ ] **Step 1: Failing test** — mapper test: a `Comparison` with an `ActivityShare` row maps into `activities[0]` with `key == "activityShare"` and `significance == "notTested"` when overlapping; reversed span params → `Err(ApiValidationError)`.
- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Implement, register route and OpenAPI paths/schemas.
- [ ] **Step 4:** `cargo test --workspace` PASS (including `openapi_contract_test`); `make check` Rust portion green.
- [ ] **Step 5:** Manual smoke: `make dev-server`, then `curl -H "Authorization: Bearer $TOKEN" 'localhost:<port>/api/v1/comparisons?aFrom=2025-10-01&aTo=2025-10-31&bFrom=2026-10-01&bTo=2026-10-31' | jq '.overview, .mood.rows[0]'` → JSON with no `NaN`. Also `aFrom=2026-10-31&aTo=2026-10-01` → 400/422 refusal shape.
- [ ] **Step 6:** Commit `feat: comparisons endpoint`.

---

### Task 8: ADR and glossary

**Files:**
- Create: `docs/adr/0021-comparing-periods-normalises-and-tests-honestly.md`
- Modify: `CONTEXT.md`

- [ ] **Step 1:** ADR in the existing ADRs' format recording: per-logged-day vs per-day-with-data denominators (weather uses days with weather data); `NotTested` for descriptive rows; no testing on overlapping spans; Benjamini–Hochberg across one comparison; separate comparison minimum sample size (10) and why (month vs month); clipping to today; trend bucketing lives on the server with span-start windows for Compare and calendar windows (client-sent week start) for the single-period trend.
- [ ] **Step 2:** `CONTEXT.md`: add **Comparison**, **PeriodSummary**, **Coverage**, **TrendBucket** entries in the house style (definition + `_Avoid_:` line).
- [ ] **Step 3:** Commit `docs: ADR for period comparison`.

---

### Task 9: SPA Compare page

**Files:**
- Run: `cd spa && bunx shadcn@latest add table tooltip toggle-group` (adds `components/ui/table.tsx`, `tooltip.tsx`, `toggle-group.tsx`)
- Modify: `spa/src/api/schema.ts` (zod for every response type of Task 7), `spa/src/api/client.ts` (`comparisons.compare(spans)`), the stats page and dashboard (link to Compare beside the metrics link)
- Create: `spa/src/routes/_auth/compare.tsx` (search-param validation: `aFrom,aTo,bFrom,bTo` as `yyyy-MM-dd`), `spa/src/routes/_auth/compare.lazy.tsx`, `spa/src/hooks/use-comparison.ts`, `spa/src/lib/compare-presets.ts`, `spa/src/components/compare/{period-picker,preset-chips,overview-section,trend-section,mood-section,activities-section,metrics-section,weather-section,music-section,cycle-section,comparison-table,comparison-row,change-bar,significance-badge}.tsx`
- Test: `spa/src/lib/compare-presets.test.ts` (vitest)

**Interfaces:**
- Consumes: `ComparisonResponse` (Task 7).
- Produces:
  - `type DaySpan = { from: string; to: string }` (dates `yyyy-MM-dd`)
  - `compare-presets.ts`: `sameMonthLastYear(today: Date): [DaySpan, DaySpan]`, `thisYearVsLastYear(today)`, `thisSeasonVsLastSeason(today)` (meteorological seasons: Dec–Feb, Mar–May, Jun–Aug, Sep–Nov; winter spans the year boundary), `previousPeriod(span: DaySpan): DaySpan` (same length, ending the day before `span.from`)
  - `useComparison(a: DaySpan, b: DaySpan)` — TanStack Query key `["comparison", a, b]`, `staleTime` 60_000, `placeholderData: keepPreviousData`
  - Colours: `PERIOD_A_COLOR`, `PERIOD_B_COLOR` constants in one place (green/blue matching the mockup), used by pickers, chart and table headers
- UI rules: layout B from the spec; `ComparisonTable` = shadcn `Table` with columns label / A / B / Δ / `ChangeBar` / `SignificanceBadge`; `SignificanceBadge` = shadcn `Badge` + `Tooltip` showing sample sizes (`likely real`, `noise`, `not enough data`; `notTested` renders no badge); shares render ×100 with `%`, share changes as `pp`; coverage `Alert` when either side's coverage < 0.5 or the two differ by > 0.3 (named constants); overlap `Alert` explaining why no marks; `cycle` section hidden when `null`; trend chart: Recharts `LineChart`, x = `offset`, two `Line type="monotone" connectNulls={false}`, tooltip shows both periods' `start` dates; activities/metrics tables sortable by |absoluteChange| client-side; default view when no search params = `sameMonthLastYear(today)`; skeletons via `Skeleton`.

- [ ] **Step 1: Failing test**

```ts
test("same month last year on 2026-11-14", () => {
  expect(sameMonthLastYear(new Date("2026-11-14"))).toEqual([
    { from: "2025-11-01", to: "2025-11-30" }, { from: "2026-11-01", to: "2026-11-30" }])
})
test("winter season spans the new year", () => {
  expect(thisSeasonVsLastSeason(new Date("2027-01-10"))[1]).toEqual({ from: "2026-12-01", to: "2027-02-28" })
})
test("previous period has the same length and ends the day before", () => {
  expect(previousPeriod({ from: "2026-10-01", to: "2026-10-31" })).toEqual({ from: "2026-08-31", to: "2026-09-30" })
})
```

- [ ] **Step 2:** `cd spa && bun run test` → FAIL.
- [ ] **Step 3:** Implement presets, schema, hook, components, route.
- [ ] **Step 4:** `bun run test && bun run lint && bun run typecheck && bun run build` → PASS.
- [ ] **Step 5:** Manual: run the app, open `/compare`, check each preset, a custom range, a bookmarked URL reload, an overlapping pair (overlap alert, no badges), a future B (header shows "so far").
- [ ] **Step 6:** Commit `feat(spa): compare page`.

---

### Task 10: Server-side trend + migrate the stats chart

**Files:**
- Create: `crates/application/src/entry/use_cases/get_mood_trend.rs`, `crates/api-types/src/params/trend.rs`, `crates/api-types/src/responses/trend.rs`, mapper in `crates/api-types/src/mappers/trend.rs`; test `crates/application/tests/entry/get_mood_trend_test.rs`
- Modify: `crates/application/src/entry/{queries.rs,use_cases/mod.rs}`, `handlers/entries.rs` (`handle_trend`), `router.rs` (`/entries/trend`), `openapi.rs`, `spa/src/api/{schema,client}.ts`, `spa/src/hooks/use-trend.ts`, `spa/src/components/stats/mood-trend-chart.tsx`
- Delete: `spa/src/lib/trend-buckets.ts`

**Interfaces:**
- Consumes: `CalendarBucketing`, `WeekStart`, `Granularity`, `TrendDay`, `TrendPoint` (Task 3); `TrendPointResponse` (Task 7).
- Produces:
  - `MoodTrendQuery { user_id, span: DateSpan, week_start: WeekStart }`; `get_mood_trend::execute(query, &Deps { entries, users }) -> Result<MoodTrend, ApplicationError>`; `MoodTrend { granularity: Granularity, points: Vec<TrendPoint> }`. No maximum span. Granularity keeps today's chart behaviour exactly: `Day` when the span is ≤ 31 days, `Week` up to 366 days, else `Month`. Add `Granularity::for_single_period(days)` beside `for_days` with its own named threshold (`LONGEST_DAILY_SINGLE = 31`).
  - `GET /api/v1/entries/trend?from&to&weekStartsOn` → `{ granularity, points: TrendPointResponse[] }`; unknown `weekStartsOn` → validation refusal; omitted → Monday.
  - SPA: `useTrend(period)` calls it with `weekStartsOn` from `lib/week.ts`; chart renders `points` (label/tooltip formatted by granularity with date-fns, as `labelsFor` did), `meanMood: null` → gap.

- [ ] **Step 1: Failing tests** — `calendar_weeks_honour_a_sunday_start`, `an_unlogged_week_is_a_gap`, `a_thirty_day_span_is_drawn_in_days`, `a_long_imported_history_is_not_refused` (6-year span → Ok).
- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Implement server side, then SPA migration; delete `trend-buckets.ts` and its imports.
- [ ] **Step 4:** `make check` → PASS.
- [ ] **Step 5:** Manual: stats page week/month/year/all look the same as before (same buckets, same tooltip entry counts).
- [ ] **Step 6:** Commit `feat: server-side mood trend`.
