use std::sync::Arc;
use std::time::Duration;

use web_push::{
    ContentEncoding, SubscriptionInfo, VapidSignatureBuilder, WebPushMessage, WebPushMessageBuilder,
};

use config::PushConfig;
use domain::errors::DomainError;
use domain::ports::PushSubscriptionQueryPort;
use domain::push::PushSubscription;
use domain::user::UserId;

pub struct WebPushSender {
    client: reqwest::Client,
    vapid_private_key: String,
    vapid_subject: String,
    subscription_query: Arc<dyn PushSubscriptionQueryPort>,
}

impl WebPushSender {
    pub fn new(
        config: &PushConfig,
        subscription_query: Arc<dyn PushSubscriptionQueryPort>,
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

    async fn deliver(&self, sub: &PushSubscription, payload: &str) -> Result<(), DomainError> {
        let subscription_info = SubscriptionInfo::new(sub.endpoint(), sub.p256dh(), sub.auth());
        let signature = self.sign_for_subscription(&subscription_info)?;
        let message = build_push_message(&subscription_info, signature, payload)?;
        let req = into_reqwest(&self.client, message);

        match req.send().await {
            Ok(response) => log_response(response, sub).await,
            Err(e) => {
                tracing::warn!(endpoint = sub.endpoint(), error = %e, "failed to send push");
            }
        }

        Ok(())
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

        for sub in &subscriptions {
            self.deliver(sub, &payload).await?;
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

async fn log_response(response: reqwest::Response, sub: &PushSubscription) {
    let status = response.status();
    if status.is_success() {
        tracing::info!(endpoint = sub.endpoint(), "push notification sent");
    } else {
        let body = response.text().await.unwrap_or_default();
        tracing::warn!(endpoint = sub.endpoint(), %status, body, "push endpoint rejected");
    }
}
