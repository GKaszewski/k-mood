#[derive(serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectProviderRequest {
    #[schema(value_type = Object)]
    pub credential: serde_json::Value,
}

impl std::fmt::Debug for ConnectProviderRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectProviderRequest")
            .field("credential", &"<redacted>")
            .finish()
    }
}
