# Clearing an account's data empties what it logged, not what it is

`delete_all_user_data` deleted four tables — entries, activities, reminders, daily metrics — and logged "all user data cleared". Cycle starts, rejected metrics and media ownership survived it, and so did every blob the account had uploaded but never attached to an entry.

The tables were missed because the account row stays, so nothing cascades. `delete_user_account` has no such gap: it deletes the users row and `foreign_keys` is on, so `ON DELETE CASCADE` reaches everything hanging off it.

## What counts as data

CompleteBackup already draws the line this needed: it carries everything an account knows and explicitly "no credentials and no RejectedMetrics". Clearing follows the same seam, one table further out.

Emptied: MoodEntries, Activities, Reminders, DailyMetrics, CycleStarts, RejectedMetrics, and the media a User owns. A RejectedMetric is a trace of the account's own readings, so clearing the readings and keeping the complaints about them would be incoherent — it goes, even though a backup does not carry it.

Kept: the account, its UserPreferences, its sessions and push subscriptions, its ApiTokens and its ProviderConnections. A preference is a choice the User made, not something they logged; clearing cycle records while leaving cycle tracking on is a coherent state and turning it off silently is not. Credentials are revoked deliberately, one at a time, and a request to clear a journal is not a request to break every automation writing into it.

## Consequences

The table list is a named constant the cascade loops over, so a new user-scoped table is one line rather than one forgotten `DELETE`. `InMemoryStore` clears the same set through one shared method, because a fake that forgets a table cannot fail the test that would have caught this.

Media is no longer resolved by composing entries and reading their photo ids. `MediaOwnershipPort::owned_by` returns everything the account owns, so a blob uploaded and abandoned before its entry was saved is cleared with the rest — the orphan that the entry-shaped question could never see. `clear_data` and `delete_user` lost their `dimensions` and `entry_query` dependencies as a result; `delete_entries_by_date_range` keeps the entry-shaped path, because it deletes a subset of entries and ownership does not say which entry a blob belongs to.

Ownership is still read before the cascade runs, for the same reason it always was: the cascade deletes the rows that name the blobs.
