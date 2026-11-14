use std::collections::BTreeMap;

use application::comparison::use_cases::compare_periods::PeriodComparison;
use domain::comparison::PeriodSummary;
use domain::entry::{ClippedSpan, DateSpan, Mood};

use crate::errors::ApiValidationError;
use crate::mappers::comparison_rows::Sections;
use crate::params::ComparedSpansParams;
use crate::responses::{
    ComparedTrendResponse, ComparisonResponse, MoodDistributionResponse, MoodFrequencyResponse,
    MoodSectionResponse, PeriodResponse,
};

use super::metric::parse_date;

impl ComparedSpansParams {
    pub fn into_spans(self) -> Result<(DateSpan, DateSpan), ApiValidationError> {
        let a = DateSpan::new(parse_date(&self.a_from)?, parse_date(&self.a_to)?)?;
        let b = DateSpan::new(parse_date(&self.b_from)?, parse_date(&self.b_to)?)?;
        Ok((a, b))
    }
}

impl From<PeriodComparison> for ComparisonResponse {
    fn from(compared: PeriodComparison) -> Self {
        let comparison = &compared.comparison;
        let sections = Sections::route(comparison.rows(), &compared.activities);
        let (trend_a, trend_b) = comparison.trend();
        let has_cycle_rows = !sections.cycle.is_empty();

        Self {
            a: period_response(&compared.spans.0, comparison.a()),
            b: period_response(&compared.spans.1, comparison.b()),
            spans_overlap: comparison.overlap(),
            trend: ComparedTrendResponse::of(comparison.granularity(), trend_a, trend_b),
            overview: sections.overview,
            mood: MoodSectionResponse {
                rows: sections.mood,
                distribution: MoodDistributionResponse {
                    a: distribution(comparison.a()),
                    b: distribution(comparison.b()),
                },
            },
            activities: sections.activities,
            metrics: sections.metrics,
            weather: sections.weather,
            music: comparison.music().into(),
            cycle: has_cycle_rows.then_some(sections.cycle),
        }
    }
}

fn period_response(span: &ClippedSpan, summary: &PeriodSummary) -> PeriodResponse {
    PeriodResponse {
        from: span.span().start().to_string(),
        to: span.span().end().to_string(),
        clipped: span.was_clipped(),
        days_in_span: summary.coverage().days_in_span,
        logged_days: summary.coverage().logged_days,
    }
}

fn distribution(summary: &PeriodSummary) -> Vec<MoodFrequencyResponse> {
    let counts: BTreeMap<Mood, usize> = summary.mood_counts();
    counts
        .into_iter()
        .map(|(mood, count)| MoodFrequencyResponse {
            mood: mood.value(),
            mood_label: mood.label().to_string(),
            count,
        })
        .collect()
}
