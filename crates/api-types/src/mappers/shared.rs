use crate::errors::ApiValidationError;

pub fn parse_datetime(
    s: &str,
) -> Result<chrono::DateTime<chrono::FixedOffset>, ApiValidationError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map_err(|e| ApiValidationError::Invalid(format!("invalid datetime: {e}")))
}

pub fn parse_uuid(s: &str) -> Result<uuid::Uuid, ApiValidationError> {
    s.parse()
        .map_err(|_| ApiValidationError::Invalid(format!("invalid UUID: {s}")))
}

pub fn parse_uuids_as<T>(
    ids: &[String],
    max: usize,
    label: &str,
) -> Result<Vec<T>, ApiValidationError>
where
    T: From<uuid::Uuid>,
{
    if ids.len() > max {
        return Err(ApiValidationError::Invalid(format!(
            "too many {label}, maximum is {max}"
        )));
    }

    ids.iter()
        .map(|s| {
            let uuid = parse_uuid(s)?;
            Ok(T::from(uuid))
        })
        .collect()
}

pub fn validate_content_length(text: &str, max: usize) -> Result<(), ApiValidationError> {
    if text.len() > max {
        return Err(ApiValidationError::Invalid(format!(
            "content exceeds maximum length of {max}"
        )));
    }
    Ok(())
}
