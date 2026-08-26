use application::provider::use_cases::list_connections::ConnectionSummary;

use crate::responses::ProviderConnectionResponse;

impl From<ConnectionSummary> for ProviderConnectionResponse {
    fn from(summary: ConnectionSummary) -> Self {
        Self {
            provider: summary.provider.value().to_string(),
            connected_at: summary.connected_at,
        }
    }
}
