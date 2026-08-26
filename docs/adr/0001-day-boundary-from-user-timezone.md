# The day boundary is resolved from the User's timezone

Day-scoped data (DailyMetric) needs a single, unambiguous notion of "which day". We resolve it by converting an instant into the User's IANA timezone and taking the local date, rather than using the offset carried on `logged_at` or a UTC date.

## Considered Options

- **Bare local date from `logged_at`** — what `get_calendar` did originally. No server-side authority: two clients in different offsets disagree about the same instant's date.
- **UTC date** — unambiguous but wrong for most humans; evening entries in eastern offsets land on the following day and fail to line up with the health data describing them.

## Consequences

`User.timezone` stops being optional in practice. Clients must set it from the platform at register/login, and requests that need a day boundary are rejected when it is unset rather than silently defaulting — silently misfiled data produces analytics that look correct and are not. `Timezone` must therefore validate against the real IANA database, not just check for a `/`.

All day grouping in the system flows through this one rule, including `get_calendar` and streak calculation, which previously used two different and mutually inconsistent definitions.

## An imported wall clock time is the User's own wall clock

Daylio's export records `2026-08-25` and `8:00 PM` and says nothing about which zone that was. The importer originally read it as UTC, so an entry logged at 8 PM in Warsaw was stored as `20:00Z` and displayed at 10 PM — every imported entry off by the User's offset, silently, and worse the further from UTC they live.

A wall clock time with no zone is the User's own wall clock. It is now placed in their Timezone using the offset in force **on that entry's own date**, not a single offset taken from today, so a year of history spanning a clock change gets both offsets. A Warsaw import shows `+01:00` in winter and `+02:00` in summer, and every entry reads as the 8 PM it was.

This makes the Timezone a precondition for importing, exactly as it already is for the calendar and for DailyMetrics: an account with none is refused rather than filed an hour or ten out of place. That is worth an error in the one place a User can act on it.

Two edge cases the clocks create:

- **An hour that never happened.** In Warsaw the clocks jump from 02:00 to 03:00 on the last Sunday in March, so a Daylio row reading 2:30 AM that day names a time that did not exist. The entry lands an hour later rather than being dropped, with a warning — a mood that was logged is not made up, and losing it to arithmetic is worse than moving it sixty minutes.
- **An hour that happened twice.** In autumn 02:30 occurs twice; the earlier of the two is used. Either is defensible and the difference is an hour once a year, so the choice is recorded rather than agonised over.

## A second bug the same file exposed

The Daylio adapter read its columns by position, and index 7 is `note_title`, not `note`. Every imported note was silently dropped and the always-empty title kept in its place. Columns are now looked up by header name, so a column added, removed or reordered upstream cannot quietly shift the meaning of the data — and an export missing `full_date`, `time` or `mood` is refused by name rather than parsed into nonsense. Where a row has both a title and a note, both are kept.
