# The cycle is correlated as progress, not as a cycle day

Cycle day is derived from the most recent preceding CycleStart and never stored, exactly as MoonPhase is derived from the calendar. What gets correlated against DayMood is progress through the cycle — 0 on its first day, 1 on its last — not the day number.

A cycle day is cyclical, so correlating it directly measures where the cycle happens to have been cut rather than anything about the person: day 1 and day 28 are neighbours in life and at opposite ends of the number line. This is the same defect that rules out correlating the moon's position through its cycle, and it is why MoonPhase is correlated as illuminated fraction instead.

## Considered Options

- **The folded transform used for MoonPhase**, `(1 - cos 2*pi*p) / 2`. Rejected here even though it is right there: it maps day 7 and day 21 to the same value, conflating the follicular and luteal halves — which is precisely the distinction anyone asking this question cares about. For the moon the fold is not a transform at all, it is the physical quantity; for a cycle it destroys the signal.
- **A sin/cos pair, correlated as two inputs.** The standard treatment of a circular variable, and it can see a symmetric effect at both ends of the cycle. Rejected for now because it doubles the comparison count in the Measurements Family for one input, and neither component means anything a User could read.

## Consequences

Progress answers "does mood drift across the cycle" and cannot answer "is mood worse at both ends". That limit is real and worth stating: a symmetric premenstrual-and-menstrual dip would show as no correlation at all. Whoever wants that question answered needs the sin/cos pair, and should know that is what it costs.

Progress needs a cycle length. A closed cycle has its own: the days until the next CycleStart. The cycle still running has none, so the median of the observed lengths is used, falling back to twenty-eight when nothing has been observed. The median rather than the mean because one missed CycleStart produces a double-length cycle, which would drag a mean and barely move a median.

Progress is clamped to 1. A cycle running late would otherwise report progress above 1, which is not a place in a cycle.

## Being off by default

The whole feature is invisible until the User turns it on, so the tracking flag gates recording, the derived cycle day on the calendar, and whether CycleProgress is a correlation input at all. Turning it off hides everything and forgets nothing — a preference is not a delete.

The flag lives on a UserPreferences record rather than as a column on User. User already carries nine fields of identity and credentials, and a display preference sitting beside the password hash is the wrong neighbourhood; the next optional feature now has somewhere to go.
