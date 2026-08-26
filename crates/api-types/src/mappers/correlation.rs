use application::correlation::use_cases::get_correlations::{CorrelationRow, Score};
use domain::correlation::CorrelationInput;

use crate::responses::{
    AgreementResponse, CorrelationInputResponse, CorrelationRowResponse, StrategyScoreResponse,
};

impl From<CorrelationRow> for CorrelationRowResponse {
    fn from(row: CorrelationRow) -> Self {
        Self {
            input: input_response(&row.input, row.label.clone()),
            sample_size: row.sample_size,
            agreement: AgreementResponse {
                agreeing: row.agreement.agreeing(),
                applicable: row.agreement.applicable(),
            },
            scores: row.scores.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Score> for StrategyScoreResponse {
    fn from(score: Score) -> Self {
        Self {
            strategy: score.strategy.name().to_string(),
            coefficient: score.coefficient.value(),
            held_up: score.held_up,
        }
    }
}

fn input_response(input: &CorrelationInput, label: Option<String>) -> CorrelationInputResponse {
    match input {
        CorrelationInput::Metric(kind) => CorrelationInputResponse::Metric {
            metric: kind.name().to_string(),
        },
        CorrelationInput::MoonPhase => CorrelationInputResponse::MoonPhase,
        CorrelationInput::CycleProgress => CorrelationInputResponse::CycleProgress,
        CorrelationInput::Temperature => CorrelationInputResponse::Temperature,
        CorrelationInput::Activity(id) => CorrelationInputResponse::Activity {
            activity_id: id.value().to_string(),
            name: label.unwrap_or_default(),
        },
    }
}
