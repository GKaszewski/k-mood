# Reading entries is one selection and one window, not four query methods

`MoodEntryQueryPort` grew a method per question: `find_by_user` with an optional limit and offset, `find_by_date_range`, `find_by_mood`, `find_by_activity`. Only the first paged, none counted, and none composed — a client could not ask for "this month's bad days, twenty at a time", and `GET /entries/filter/mood/5` returned every matching entry an account had ever written.

Without a count, `GET /entries` could only be paged by fetching until a short page came back, and a client polling for changes had to refetch the whole journal because `updatedAt` was returned but never queryable.

## Considered Options

- **Add limit, offset and a count to each existing method** — four signatures growing the same four parameters, and still no way to combine two narrowings.
- **Add the paging methods beside the old ones** — no risk to the internal readers, and two ways to ask the same question, which is the shape this document exists to remove.

## Consequences

`EntrySelection` names what to read — account, date range, mood, activity, changed-since — and `Pagination` names the window over it. The port offers `select` and `count` against that pair, and one SQL builder assembles both from the same conditions, so a filter can never apply to the page and not the count.

`find_by_mood` and `find_by_activity` are gone; they were only ever the API's. `find_by_user` became `find_all_by_user` with no window at all, because every remaining caller — backup, extract, import, restore, stats — wants the whole account and said so by passing `None, None`. The two paths are now honestly different: one page of a selection, or everything.

`Pagination::new` refuses a page of nothing, a negative offset, and a page larger than the server serves, so an unbounded read is not expressible rather than merely discouraged. The bound is configuration, reported by `GET /api/v1/server`, not a constant.

`Page` carries the total for the whole selection alongside the items, so `hasMore` is a fact rather than an inference from a short page.

`/entries/filter/mood/{mood}` and `/entries/filter/activity/{id}` stay, and now delegate to the same handler with one parameter preset. They are a convenience over the selection, not a second way to read.

## What a polling client should do

`updatedSince` returns only entries changed after an instant, which is why `updated_at` is indexed. A widget refreshing every minute asks for what changed rather than for the journal, and the answer is usually empty.

This is a poll, not a subscription: an entry deleted since the last poll leaves no trace in the result, so a client that must notice deletions still needs an occasional full read. Recording tombstones to make deletion observable is real work for a case a personal journal has not yet needed.
