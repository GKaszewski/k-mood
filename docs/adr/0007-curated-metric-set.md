# The metric set is deliberately small

Only metrics that can carry signal are stored: steps, sleep minutes, awake minutes, resting heart rate, HRV, exercise minutes, screen time, and alcoholic drinks — plus CycleStart, Weather, and MoonPhase. The originally proposed set was roughly twice this size.

Every MetricKind added costs an expected false positive across the correlation set, so a shorter list is what makes the surviving results credible.

## Rejected, and why

- **Blood oxygen, respiratory rate** — near-constant in healthy people at sea level. Most of their day-to-day movement is measurement error, and nothing can correlate against a constant.
- **Sleep stage splits (deep, REM, light)** — wrist-based sleep *staging* is weakly validated against polysomnography, unlike sleep/wake detection which is reasonable. Three low-validity metrics inflating the comparison count in the one area already covered well by sleep minutes. Awake minutes is kept because fragmentation is detected far more reliably than stage.
- **Active energy** — a device estimate derived largely from steps and exercise minutes, both already present.
- **Average heart rate** — confounded by activity; high precisely on days exercise minutes already explains.
- **Mindful minutes** — zero for most people on most days. No variance, no correlation.
- **Water intake** — self-reported, remembered badly, logged inconsistently.
- **Weight** — day-to-day movement is water, not signal. Meaningful over months, noise over days.
- **Caffeine** — dropped for a simpler reason: it carries a unit conflict with no clean answer. Providers report milligrams; the manual input is "a coffee". Converting between them is fiction, since drip, espresso, and energy drinks span roughly threefold per serving. Alcohol has no such problem — providers already report a count of standard drinks, so a tap and an import agree natively.

## Consequences

Storing and analysing are the same set; nothing is collected "just in case". Adding a MetricKind later is one enum variant, and HealthKit retains history on-device indefinitely, so a future importer can backfill years for a newly-added kind — the capability is not lost, only the convenience.

MoonPhase is kept despite having no plausible mechanism. It is a control: a strategy set reporting a strong lunar effect is reporting its own false-positive rate.
