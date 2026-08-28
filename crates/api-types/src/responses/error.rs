#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: ErrorDetailResponse,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetailResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            error: ErrorDetailResponse {
                code: code.to_string(),
                message: message.into(),
            },
        }
    }
}
