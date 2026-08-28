use domain::push::PushSubscription;

use crate::responses::PushSubscriptionResponse;

impl From<PushSubscription> for PushSubscriptionResponse {
    fn from(subscription: PushSubscription) -> Self {
        Self {
            id: subscription.id().value().to_string(),
            endpoint: subscription.endpoint().to_string(),
            created_at: *subscription.created_at(),
        }
    }
}
