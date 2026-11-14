use crate::activity::ActivityId;
use crate::entry::Mood;
use crate::metric::MetricKind;
use crate::weather::Condition;

use super::CycleQuarter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantity {
    DaysInSpan,
    LoggedDays,
    Coverage,
    Entries,
    EntriesPerLoggedDay,
    AverageMood,
    LongestLoggingStreak,
    LongestGoodStreak,
    MoodShare(Mood),
    ActivityShare(ActivityId),
    MoodWithActivity(ActivityId),
    MetricMean(MetricKind),
    MetricDaysWithData(MetricKind),
    MetricTotal(MetricKind),
    WeatherShare(Condition),
    MeanTemperature,
    MoodInWeather(Condition),
    MoodInCycleQuarter(CycleQuarter),
}
