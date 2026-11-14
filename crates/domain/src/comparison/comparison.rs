use crate::trend::{Granularity, SpanStartBucketing, TrendBucketing, TrendDay, TrendPoint};

use super::measurements::Measurements;
use super::significance_judge::SignificanceJudge;
use super::{
    ComparedSpans, ComparisonRow, ComparisonRules, Measure, MusicComparison, PeriodSummary,
    Quantity,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    a: PeriodSummary,
    b: PeriodSummary,
    overlap: bool,
    rows: Vec<ComparisonRow>,
    music: MusicComparison,
    trend: (Vec<TrendPoint>, Vec<TrendPoint>),
    granularity: Granularity,
}

impl Comparison {
    pub fn between(
        a: PeriodSummary,
        b: PeriodSummary,
        spans: &ComparedSpans,
        trend_a: &[TrendDay],
        trend_b: &[TrendDay],
        rules: ComparisonRules,
    ) -> Self {
        let overlap = spans.overlap();
        let rows = Self::rows_between(&a, &b, SignificanceJudge::new(rules, overlap));
        let music = MusicComparison::between(a.music().clone(), b.music().clone());
        let granularity = Self::shared_granularity(&a, &b);
        let trend = (
            SpanStartBucketing.bucket(a.span().span(), trend_a, granularity),
            SpanStartBucketing.bucket(b.span().span(), trend_b, granularity),
        );

        Self {
            a,
            b,
            overlap,
            rows,
            music,
            trend,
            granularity,
        }
    }

    fn rows_between(
        a: &PeriodSummary,
        b: &PeriodSummary,
        judge: SignificanceJudge,
    ) -> Vec<ComparisonRow> {
        let measured: Vec<(Quantity, Measure)> = Measurements::of(a, b).all();
        let measures: Vec<&Measure> = measured.iter().map(|(_, measure)| measure).collect();
        let significances = judge.judge(&measures);

        measured
            .iter()
            .zip(significances)
            .map(|((quantity, measure), significance)| {
                ComparisonRow::new(quantity.clone(), measure, significance)
            })
            .collect()
    }

    fn shared_granularity(a: &PeriodSummary, b: &PeriodSummary) -> Granularity {
        let longer = a.span().span().days().max(b.span().span().days());
        Granularity::for_days(longer)
    }
}

impl Comparison {
    pub fn a(&self) -> &PeriodSummary {
        &self.a
    }

    pub fn b(&self) -> &PeriodSummary {
        &self.b
    }

    pub fn overlap(&self) -> bool {
        self.overlap
    }

    pub fn rows(&self) -> &[ComparisonRow] {
        &self.rows
    }

    pub fn music(&self) -> &MusicComparison {
        &self.music
    }

    pub fn trend(&self) -> (&[TrendPoint], &[TrendPoint]) {
        (&self.trend.0, &self.trend.1)
    }

    pub fn granularity(&self) -> Granularity {
        self.granularity
    }
}
