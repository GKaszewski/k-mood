use application::restore::use_cases::restore_backup::RestoreOutcome;

use crate::responses::RestoreOutcomeResponse;

impl From<RestoreOutcome> for RestoreOutcomeResponse {
    fn from(outcome: RestoreOutcome) -> Self {
        Self {
            entries: outcome.entries,
            metrics: outcome.metrics,
            cycle_starts: outcome.cycle_starts,
            activities: outcome.activities,
            reminders: outcome.reminders,
            media: outcome.media,
            unreadable: outcome.unreadable,
        }
    }
}
