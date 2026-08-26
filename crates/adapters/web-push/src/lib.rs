use std::sync::Arc;
use std::time::Duration;

use web_push::{
    ContentEncoding, SubscriptionInfo, VapidSignatureBuilder, WebPushMessage, WebPushMessageBuilder,
};

use config::PushConfig;
use domain::errors::DomainError;
use domain::ports::{PushSubscriptionCommandPort, PushSubscriptionQueryPort};
use domain::push::PushSubscription;
use domain::user::UserId;

pub struct WebPushSender {
    client: reqwest::Client,
    vapid_private_key: String,
    vapid_subject: String,
    subscription_query: Arc<dyn PushSubscriptionQueryPort>,
    subscription_command: Arc<dyn PushSubscriptionCommandPort>,
}

/// What happened to a single subscription during a send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delivery {
    Sent,
    /// The subscription is permanently unusable and has been dropped.
    Removed,
    /// Transient failure; the subscription is kept for the next attempt.
    Failed,
}

impl WebPushSender {
    pub fn new(
        config: &PushConfig,
        subscription_query: Arc<dyn PushSubscriptionQueryPort>,
        subscription_command: Arc<dyn PushSubscriptionCommandPort>,
    ) -> Result<Self, DomainError> {
        let private_key = required_config(&config.vapid_private_key, "vapid_private_key")?;
        let subject = required_config(&config.vapid_subject, "vapid_subject")?;

        validate_vapid_key(private_key)?;

        let client = build_http_client()?;

        Ok(Self {
            client,
            vapid_private_key: private_key.to_string(),
            vapid_subject: subject.to_string(),
            subscription_query,
            subscription_command,
        })
    }

    pub fn public_key_base64(config: &PushConfig) -> Result<String, DomainError> {
        let private_key = required_config(&config.vapid_private_key, "vapid_private_key")?;

        let partial = VapidSignatureBuilder::from_base64_no_sub(private_key)
            .map_err(|e| DomainError::InvalidInput(format!("invalid VAPID key: {e}")))?;

        Ok(base64_url_encode(&partial.get_public_key()))
    }

    fn sign_for_subscription(
        &self,
        subscription_info: &SubscriptionInfo,
    ) -> Result<web_push::VapidSignature, DomainError> {
        let mut sig_builder =
            VapidSignatureBuilder::from_base64(&self.vapid_private_key, subscription_info)
                .map_err(|e| {
                    DomainError::InvalidInput(format!("failed to build VAPID signature: {e}"))
                })?;

        sig_builder.add_claim("sub", &*self.vapid_subject);

        sig_builder
            .build()
            .map_err(|e| DomainError::InvalidInput(format!("failed to sign push message: {e}")))
    }

    async fn deliver(&self, sub: &PushSubscription, payload: &str) -> Delivery {
        // Subscriptions stored before the SPA switched to base64url hold standard
        // base64 keys; web-push only accepts base64url, so normalise on read.
        let subscription_info = SubscriptionInfo::new(
            sub.endpoint().to_string(),
            normalize_base64url(sub.p256dh()),
            normalize_base64url(sub.auth()),
        );

        let message = match self
            .sign_for_subscription(&subscription_info)
            .and_then(|sig| build_push_message(&subscription_info, sig, payload))
        {
            Ok(message) => message,
            Err(e) => {
                tracing::warn!(
                    endpoint = sub.endpoint(),
                    error = %e,
                    "push subscription has unusable keys, dropping it"
                );
                self.forget(sub).await;
                return Delivery::Removed;
            }
        };

        match into_reqwest(&self.client, message).send().await {
            Ok(response) => self.handle_response(response, sub).await,
            Err(e) => {
                tracing::warn!(endpoint = sub.endpoint(), error = %e, "failed to send push");
                Delivery::Failed
            }
        }
    }

    async fn handle_response(
        &self,
        response: reqwest::Response,
        sub: &PushSubscription,
    ) -> Delivery {
        let status = response.status();

        if status.is_success() {
            tracing::info!(endpoint = sub.endpoint(), "push notification sent");
            return Delivery::Sent;
        }

        let body = response.text().await.unwrap_or_default();
        tracing::warn!(endpoint = sub.endpoint(), %status, body, "push endpoint rejected");

        // 404/410 mean the push service has retired this endpoint for good.
        if status == reqwest::StatusCode::NOT_FOUND || status == reqwest::StatusCode::GONE {
            self.forget(sub).await;
            return Delivery::Removed;
        }

        Delivery::Failed
    }

    async fn forget(&self, sub: &PushSubscription) {
        if let Err(e) = self
            .subscription_command
            .delete_by_endpoint(sub.endpoint())
            .await
        {
            tracing::warn!(endpoint = sub.endpoint(), error = %e, "failed to drop push subscription");
        }
    }
}

#[async_trait::async_trait]
impl domain::ports::ReminderSenderPort for WebPushSender {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError> {
        let subscriptions = self.subscription_query.find_by_user(user_id).await?;

        if subscriptions.is_empty() {
            tracing::debug!(%user_id, "no push subscriptions, skipping");
            return Ok(());
        }

        let payload = reminder_payload();
        let mut sent = 0usize;
        let mut removed = 0usize;
        let mut failed = 0usize;

        // One broken subscription must never stop the others from being delivered.
        for sub in &subscriptions {
            match self.deliver(sub, &payload).await {
                Delivery::Sent => sent += 1,
                Delivery::Removed => removed += 1,
                Delivery::Failed => failed += 1,
            }
        }

        tracing::info!(%user_id, sent, removed, failed, "push reminder dispatched");

        if sent == 0 {
            return Err(DomainError::InvalidInput(format!(
                "no push notification could be delivered ({removed} stale subscription(s) dropped, \
                 {failed} failed); re-enable notifications on your devices"
            )));
        }

        Ok(())
    }
}

fn required_config<'a>(value: &'a Option<String>, name: &str) -> Result<&'a str, DomainError> {
    value
        .as_deref()
        .ok_or_else(|| DomainError::InvalidInput(format!("{name} is required")))
}

fn validate_vapid_key(private_key: &str) -> Result<(), DomainError> {
    VapidSignatureBuilder::from_base64_no_sub(private_key)
        .map_err(|e| DomainError::InvalidInput(format!("invalid VAPID key: {e}")))?;
    Ok(())
}

fn build_http_client() -> Result<reqwest::Client, DomainError> {
    reqwest::Client::builder()
        .pool_max_idle_per_host(2)
        .pool_idle_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| DomainError::InvalidInput(format!("failed to create HTTP client: {e}")))
}

fn base64_url_encode(input: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

/// Rewrite a standard-base64 key into the unpadded base64url form web-push expects.
/// Already-base64url input passes through unchanged.
fn normalize_base64url(key: &str) -> String {
    key.trim()
        .trim_end_matches('=')
        .chars()
        .map(|c| match c {
            '+' => '-',
            '/' => '_',
            other => other,
        })
        .collect()
}

fn reminder_payload() -> String {
    serde_json::json!({
        "title": "K-Mood",
        "body": "How are you feeling right now?",
        "url": "/"
    })
    .to_string()
}

fn build_push_message(
    subscription_info: &SubscriptionInfo,
    signature: web_push::VapidSignature,
    payload: &str,
) -> Result<WebPushMessage, DomainError> {
    let mut builder = WebPushMessageBuilder::new(subscription_info);
    builder.set_payload(ContentEncoding::Aes128Gcm, payload.as_bytes());
    builder.set_vapid_signature(signature);

    builder
        .build()
        .map_err(|e| DomainError::InvalidInput(format!("failed to build push message: {e}")))
}

fn into_reqwest(client: &reqwest::Client, message: WebPushMessage) -> reqwest::RequestBuilder {
    let mut req = client
        .post(message.endpoint.to_string())
        .header("TTL", message.ttl.to_string());

    if let Some(urgency) = message.urgency {
        req = req.header("Urgency", urgency.to_string());
    }

    if let Some(topic) = message.topic {
        req = req.header("Topic", topic);
    }

    if let Some(payload) = message.payload {
        req = req
            .header("Content-Encoding", payload.content_encoding.to_str())
            .header("Content-Type", "application/octet-stream");

        for (k, v) in payload.crypto_headers {
            req = req.header(k, v);
        }

        req = req.body(payload.content);
    }

    req
}

#[cfg(test)]
mod tests {
    use super::*;

    // A real browser key, in both encodings the SPA has used.
    const P256DH_URL: &str =
        "BIM-sgr6FjcgJEXAWF1FysKt3Ua19-HfezOPbDVCpvQvGjvH4ITJCVqIZycQO9LFrJA3cLwDgZzf__tpMu9sJMs";
    const P256DH_STD: &str =
        "BIM+sgr6FjcgJEXAWF1FysKt3Ua19+HfezOPbDVCpvQvGjvH4ITJCVqIZycQO9LFrJA3cLwDgZzf//tpMu9sJMs=";
    const AUTH_URL: &str = "q5Ph0_85t6y2GhRWxzDnyw";
    const AUTH_STD: &str = "q5Ph0/85t6y2GhRWxzDnyw==";
    const ENDPOINT: &str = "https://jmt17.google.com/fcm/send/eLUei3g4D0M";

    #[test]
    fn normalizes_legacy_standard_base64_keys() {
        assert_eq!(normalize_base64url(P256DH_STD), P256DH_URL);
        assert_eq!(normalize_base64url(AUTH_STD), AUTH_URL);
    }

    #[test]
    fn leaves_base64url_keys_untouched() {
        assert_eq!(normalize_base64url(P256DH_URL), P256DH_URL);
        assert_eq!(normalize_base64url(AUTH_URL), AUTH_URL);
    }

    /// Legacy rows used to abort the whole send with "invalid cryptographic keys".
    #[test]
    fn legacy_keys_encrypt_after_normalization() {
        for (p256dh, auth) in [(P256DH_STD, AUTH_STD), (P256DH_URL, AUTH_URL)] {
            let info = SubscriptionInfo::new(
                ENDPOINT.to_string(),
                normalize_base64url(p256dh),
                normalize_base64url(auth),
            );
            let payload = reminder_payload();
            let mut builder = WebPushMessageBuilder::new(&info);
            builder.set_payload(ContentEncoding::Aes128Gcm, payload.as_bytes());
            assert!(builder.build().is_ok(), "failed for {p256dh}");
        }
    }

    #[test]
    fn raw_standard_base64_keys_are_rejected_by_web_push() {
        let info = SubscriptionInfo::new(ENDPOINT, P256DH_STD, AUTH_STD);
        let payload = reminder_payload();
        let mut builder = WebPushMessageBuilder::new(&info);
        builder.set_payload(ContentEncoding::Aes128Gcm, payload.as_bytes());
        assert!(builder.build().is_err());
    }
}
