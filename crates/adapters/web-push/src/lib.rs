use std::sync::Arc;

use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

use config::PushConfig;
use domain::errors::DomainError;
use domain::ports::PushSubscriptionQueryPort;
use domain::user::UserId;

pub struct WebPushSender {
    client: IsahcWebPushClient,
    vapid_private_key: Vec<u8>,
    vapid_subject: String,
    subscription_query: Arc<dyn PushSubscriptionQueryPort>,
}

impl WebPushSender {
    pub fn new(
        config: &PushConfig,
        subscription_query: Arc<dyn PushSubscriptionQueryPort>,
    ) -> Result<Self, DomainError> {
        let private_key = config
            .vapid_private_key
            .as_deref()
            .ok_or_else(|| DomainError::InvalidInput("vapid_private_key is required".into()))?;

        let subject = config
            .vapid_subject
            .as_deref()
            .ok_or_else(|| DomainError::InvalidInput("vapid_subject is required".into()))?;

        let decoded = base64_decode(private_key)?;

        let client = IsahcWebPushClient::new()
            .map_err(|e| DomainError::InvalidInput(format!("failed to create push client: {e}")))?;

        Ok(Self {
            client,
            vapid_private_key: decoded,
            vapid_subject: subject.to_string(),
            subscription_query,
        })
    }

    pub fn public_key_base64(config: &PushConfig) -> Result<String, DomainError> {
        let private_key = config
            .vapid_private_key
            .as_deref()
            .ok_or_else(|| DomainError::InvalidInput("vapid_private_key is required".into()))?;

        let decoded = base64_decode(private_key)?;

        let sig_builder = VapidSignatureBuilder::from_pem_no_sub(std::io::Cursor::new(&decoded))
            .map_err(|e| DomainError::InvalidInput(format!("invalid VAPID key: {e}")))?;

        let public_key = sig_builder.get_public_key();
        Ok(base64_url_encode(&public_key))
    }
}

fn base64_decode(input: &str) -> Result<Vec<u8>, DomainError> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|e| DomainError::InvalidInput(format!("invalid base64: {e}")))
}

fn base64_url_encode(input: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

#[async_trait::async_trait]
impl domain::ports::ReminderSenderPort for WebPushSender {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError> {
        let subscriptions = self.subscription_query.find_by_user(user_id).await?;

        if subscriptions.is_empty() {
            tracing::debug!(%user_id, "no push subscriptions, skipping");
            return Ok(());
        }

        let payload = serde_json::json!({
            "title": "K-Mood",
            "body": "How are you feeling right now?",
            "url": "/"
        });
        let payload_str = payload.to_string();

        for sub in &subscriptions {
            let subscription_info = SubscriptionInfo::new(sub.endpoint(), sub.p256dh(), sub.auth());

            let mut sig_builder = VapidSignatureBuilder::from_pem(
                std::io::Cursor::new(&self.vapid_private_key),
                &subscription_info,
            )
            .map_err(|e| {
                DomainError::InvalidInput(format!("failed to build VAPID signature: {e}"))
            })?;

            sig_builder.add_claim("sub", &*self.vapid_subject);
            let signature = sig_builder.build().map_err(|e| {
                DomainError::InvalidInput(format!("failed to sign push message: {e}"))
            })?;

            let mut builder = WebPushMessageBuilder::new(&subscription_info);
            builder.set_payload(ContentEncoding::Aes128Gcm, payload_str.as_bytes());
            builder.set_vapid_signature(signature);

            let message = builder.build().map_err(|e| {
                DomainError::InvalidInput(format!("failed to build push message: {e}"))
            })?;

            match self.client.send(message).await {
                Ok(_) => {
                    tracing::info!(%user_id, endpoint = sub.endpoint(), "push notification sent");
                }
                Err(e) => {
                    tracing::warn!(%user_id, endpoint = sub.endpoint(), error = %e, "failed to send push");
                }
            }
        }

        Ok(())
    }
}
