# Correlations are shown as agreement across strategies, never as ranked findings

Several CorrelationStrategies (Pearson, Spearman, Kendall, mean difference) are computed over the same data and presented side by side. Agreement among them is the headline; no single coefficient is authoritative.

Results are deliberately **not** ranked, not badged with significance, and not surfaced as an "insights" feed.

## Considered Options

- **A ranked insights list with p-values** — by far the most engaging presentation, and rejected for that reason. With roughly 25 MetricKinds tested at conventional thresholds, more than one spurious result is expected per user per run; ranking by strength selects precisely for whichever metric got lucky, and a p-value badge converts that coincidence into a claim someone may act on.
- **Strict multiple-comparison correction as the only view** — statistically the most defensible, but with this many metrics and a few months of data essentially nothing survives, so the feature reads as broken rather than rigorous and users conclude the tracking was pointless.

## Consequences

Showing several strategies is a robustness check, not a menu. A correlation that holds under all of them is worth attention; one that appears under a single strategy is reporting an outlier or a non-linearity. Presenting them as choices would let a user shop for the flattering number, which is worse than showing one.

Strategies use rank-based methods where possible because Mood is ordinal — the domain asserts that Rad is better than Good, but never that the gap from Awful to Bad equals the gap from Good to Rad, which is what Pearson assumes.

A minimum sample size gates output entirely: below it, no coefficient is returned rather than one computed from a handful of days.

Adjustment is a distinct layer over a set of results, not a strategy. Anything that cannot compute a coefficient for a single metric-mood pair on its own does not belong in the strategy set.

Adjustment is Benjamini-Hochberg, controlling false discovery rate, and it marks results rather than hiding them. Nothing is gated on surviving it. A surviving result is presented as a second axis of robustness beside cross-strategy agreement — it held up once the number of things tested was accounted for — never as a verdict or a significance claim.

FDR is the right family here rather than family-wise error control. A false positive costs a user a behaviour change that achieves nothing; a false negative costs them a real and actionable effect they never discover. In a personal exploratory tool the second is worse, and strict family-wise control needs roughly |r| >= 0.34 over ninety days, which would leave the page empty for months.

Correction applies within a CorrelationStrategy, never across strategies. Several strategies are not several hypotheses — they are several measurements of one, so correcting across them would penalise measuring carefully.

MetricKinds and Activities are corrected as separate Families. A Family is a question, and how many Activities a User chooses to keep has no bearing on whether their sleep tracks their mood — a single Family would let a large Activity catalog quietly suppress metric findings, making the guardrail's strictness depend on a number the User controls for unrelated reasons.

The q threshold and the minimum sample size are configuration, not constants.

MoonPhase is correlated like anything else and is expected to show nothing. It is included because people enjoy it, and it doubles as a control: a strategy set that reports a strong lunar effect is reporting its own false-positive rate.

## What the numbers are

Spearman is computed as Pearson over average ranks, not with the textbook `1 - 6*d^2/(n(n^2-1))` shortcut. That shortcut is only valid when no ranks are tied, and Mood has five levels, so a hundred days of entries are almost entirely ties. On a small worked example the two disagree in the third decimal, and the disagreement grows with the number of ties — it would be a quietly wrong number rather than a visibly broken one.

MoonPhase is correlated as illuminated fraction, from 0 at new to 1 at full, not as position through the cycle. A cycle position is circular: 0.99 and 0.01 are a day apart in the sky and at opposite ends of the number line, so any linear or rank correlation over it measures an artefact of where the cycle was cut. Illumination is monotonic in what a lunar-effect claim is actually about, which is how bright the night is.

A strategy returns nothing rather than a number when either series never varies. A user who logs the same mood every day has no correlation to report, and zero would assert that the metric was tested and found unrelated.

## Comparing across the strategy set

Mean difference is reported as a share of the mood scale rather than in mood points: the difference of the two means is divided by the span from Awful to Rad, so a full mood step reads as 0.25 and the number sits in the same -1..1 range as the three coefficients. Every strategy therefore returns one type with one range, which is what makes a row of them readable side by side. The cost is that "+0.25" no longer says "a quarter of a mood point"; the display has to say what a quarter of the scale means.

Agreement is the size of the largest group of returned coefficients sharing a sign, out of the number of strategies that could score the input at all. Reporting the denominator matters: an Activity can only be scored by mean difference, so it is always one of one, and presenting that as unanimity would imply corroboration that does not exist. The display says "one measure only" instead.

An Activity with no logged day inside the span is not returned. A row with a day count and no coefficient reads as "measured, nothing found"; for an activity the User never tagged, nothing was measured. Preset catalogs make this the common case rather than an edge one — a new account carries two dozen activities it has never used.

## How the adjustment gets its p-values

Benjamini-Hochberg needs a p-value per result, and this document forbids p-values reaching the response or the UI. They are therefore computed, used to decide the mark, and discarded inside the use case; the public result type has no field for one.

They come from normal approximations rather than exact distributions: Fisher's z transform for Pearson and Spearman — with the 1.06 variance inflation for the rank version — the usual large-sample z for Kendall's tau, and Welch's z for mean difference, all through one hand-rolled error function. The domain crate takes no dependencies, so the alternative was an incomplete beta function and a log-gamma, roughly triple the numerical code in exactly the class that is subtly wrong in ways mid-range tests miss. These approximations are sound from around twenty-five paired days and the minimum sample size is thirty, so the error is orders of magnitude below anything that changes a mark. Lowering that floor is the change that would make the approximation the wrong choice.

## Correcting within a group

The grouping rule lives with the Adjustment rather than at the call site: it takes results tagged with their Family and their CorrelationStrategy and corrects within each pair. Grouping by Family and Strategy is today indistinguishable from grouping by Strategy alone, because every Activity is scored by mean difference and no measurement is — the two partitions coincide. It is written and tested as both because the coincidence is an accident of the current strategy set, and a point-biserial correlation over Activity presence would end it without touching this code.

Note the direction of BH's step-up: adding results with small p-values raises the threshold every other result is judged against, so a larger set does not simply make each result work harder. That is why correcting across strategies is not merely conservative — measuring one relationship three ways would let the strongest of the three pull the other two through.
