# Comparing periods normalises first and tests only what can be tested

A Comparison puts two DateSpans side by side. Every compared figure is either a rate or a descriptive plain figure, and no raw total is ever tested; a difference is marked as likely real only when it could not plausibly be noise. All of it is computed on the server; a client receives identical numbers and re-implements nothing.

## Considered Options

- **Raw totals side by side** — the simplest table, and misleading whenever the spans differ in length or in how much was logged. Ninety days against thirty reads as a threefold change in everything.
- **A difference with no mark** — honest about the numbers, silent about whether they mean anything. With dozens of rows, some will differ by chance, and an unmarked difference invites the same shopping for a flattering row that ADR 0006 rules out.
- **Testing every row, however thin** — always gives the page something to say, and says it with a confidence the data cannot support.

## Consequences

Each row is divided by the number of days that could have carried it. Mood, activities and mood shares are per logged day. Weather is per day that has weather data: weather is observed after the fact and can be missing, and dividing by all logged days would read a gap in the observations as a change in the weather. Health metrics are the mean per day with data, for the same reason. The response reports the denominator beside each figure.

A row is one of four things: `LikelyReal`, `Noise`, `NotEnoughData` and `NotTested`. `NotTested` is for descriptive rows with no sampling question behind them — coverage, logged days, entries, streaks, metric totals. They are shown as plain figures with a change, never labelled noise, and take no part in the adjustment.

`NotTested` is also the answer when both samples are large enough but the test cannot score because nothing varies, such as a mood felt on none of 30 days in either period. That is a statement about the measurement, not about a shortage of days, and `NotEnoughData` would send the User to log more of something that was never going to vary.

When the spans overlap, no row is tested and the response says so. The tests assume independent samples, and the same day counted on both sides would make a difference look more certain than it is. Overlap is still allowed, since "this year so far" against "the last ninety days" is a fair question; it is simply answered with figures and no marks.

Means (mood, metrics, temperature) use Welch's test on the per-day values. Shares use a two-proportion z-test, and only when every cell's expected count is at least 5. Below that the row is `NotEnoughData`. The normal approximation overstates significance on small counts: 0 of 10 against 4 of 10 gives p of about 0.025 where Fisher's exact test gives about 0.087. A false "likely real" is the failure that matters most here, so the test declines rather than guess. This is the same reasoning ADR 0006 gives for the minimum sample size, applied per cell.

A mean row turns Welch's statistic into a p-value with Student's t distribution on Welch–Satterthwaite degrees of freedom, not the normal tail. Compare's minimum of 10 days per side is too small for the normal approximation, which overstates significance there: t of 2.3 reads as p of about 0.021 under the normal curve, against about 0.034 on 18 degrees of freedom and about 0.047 on 9. Correlations keep the normal tail, since ADR 0006's reasoning holds at their minimum of 30.

Benjamini-Hochberg runs once across every tested row of one Comparison, at the configured rate. A row is `LikelyReal` only if it survives. As in ADR 0006 this is false discovery rate rather than family-wise control, for the same reason. Correlations are not badged with significance (ADR 0006); comparison rows are, because a Comparison asks one question of two periods and the User needs to know which differences to believe; the p-values themselves still never leave the use case.

Compare has its own minimum sample size, `analysis.comparison_minimum_sample_size`, default 10, which replaces the shared gate for comparisons and leaves the correlations' 30 alone. Correlation is gated at 30 because it needs paired days to say anything about a relationship. A month against a month holds at most 31 logged days per side, and a gate of 30 would mark nearly every row `NotEnoughData`. Ten is low enough that a normal month passes while a very quiet one is still declined, and high enough for a mean difference to be tested.

A span that ends after the User's today is clipped to today, in the User's Timezone, before anything is summarised. Days in the future are never counted as unlogged, which would drag coverage and every per-day rate down for no reason. The response reports the effective span so a client can say "so far".

## Where trend buckets are made

A TrendBucket is built on the server, by the same domain code for every client, replacing the bucketing that lived in the SPA. A trend point then means one thing everywhere, and a second client does not need to copy it.

There are two bucketings because they answer different questions. Compare counts buckets from each span's own start, in fixed windows of 1, 7 or 30 days, so bucket *n* of A and bucket *n* of B cover the same number of days and the overlay is like for like. A calendar month would break that: February would be set against March. The single-period trend keeps calendar days, weeks and months, with the week start sent by the client from its locale, so the existing chart does not change for anyone. An empty bucket has no mean rather than an interpolated one, so a chart shows a gap instead of inventing a value.
