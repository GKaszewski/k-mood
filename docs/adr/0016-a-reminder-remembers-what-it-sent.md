# A Reminder remembers the occurrence it sent

`process_due_reminders` decided a reminder was due by subtracting the scheduled time from the local wall clock and accepting a difference of nought to five minutes. The worker polls every sixty seconds. Every reminder therefore fired five times, once per tick, and `Reminder` held no state that could tell the second tick the first had already sent.

The same subtraction failed across midnight. A reminder set for 23:50 was compared against a 00:05 wall clock as a `NaiveTime` difference, which is large and negative, so the sweep that should have delivered it skipped it instead.

## Considered Options

- **Widen the poll interval to match the window** — one line, and it trades duplicate sends for a reminder arriving up to five minutes late while still fixing nothing about midnight. The window and the interval stay coupled, so tuning either reintroduces the bug.
- **A sent-today marker keyed on the local date** — enough for one reminder a day, and wrong for the two-reminders-a-day case the domain language already allows ("one for morning check-in, one for evening").

## Consequences

`Reminder` carries `last_sent_at`, and the question it answers is not "is it roughly this time" but "which scheduled occurrence has been reached, and was that one already sent". `occurrence_reached` resolves the scheduled wall clock into the User's zone, takes it as an instant, and offers it only when the instant has passed, is inside the grace window, and is later than whatever was last sent. `mark_sent` stores the occurrence's own instant, not the moment of sending, so the comparison is against the schedule rather than against the clock.

Yesterday's local date is considered alongside today's, which is what carries a 23:50 reminder over the midnight boundary.

The window is a configured grace rather than a constant, defaulting to thirty minutes. It bounds how late a delivery may still be attempted — a worker restarted at noon does not fire the morning's reminder — and it no longer has to agree with the poll interval, because the duplicate is prevented by the recorded occurrence and not by the arithmetic.

A send that fails is not marked, so the next sweep inside the window tries again. A send that succeeds but whose write fails is logged at error and may repeat, which is the right way round: this is a notification, and one arriving twice costs less than one that never arrives.

## The sweep no longer stops for one account

The loop carried a comment saying one user with no reachable device must not stop the sweep for everyone behind them, and it was half true. A failing `send_reminder` was caught, but a failing `find_by_id` propagated with `?` and abandoned the rest. Reading the account behind a reminder now degrades the same way delivering to it does: logged, skipped, sweep continues.
