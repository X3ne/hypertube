use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

pub(super) type Result<T> = std::result::Result<T, MetadataError>;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Failed to authenticate with metadata provider")]
    AuthError,
    #[error("Invalid id")]
    InvalidId,
    #[error("Failed to get metadata")]
    FailedToGetMetadata,
    #[error("Missing metadata")]
    MissingMetadata,
    #[error("Failed to parse metadata")]
    FailedToParseMetadata,
    #[error("Failed to search show")]
    SearchError,
    #[error("Missing slug")]
    MissingSlug,
}

impl ApiErrorImpl for MetadataError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            MetadataError::AuthError => (StatusCode::UNAUTHORIZED, "auth_error"),
            MetadataError::InvalidId => (StatusCode::BAD_REQUEST, "invalid_id"),
            MetadataError::FailedToGetMetadata => {
                (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_get_metadata")
            }
            MetadataError::MissingMetadata => (StatusCode::NOT_FOUND, "missing_metadata"),
            MetadataError::FailedToParseMetadata => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed_to_parse_metadata",
            ),
            MetadataError::SearchError => (StatusCode::INTERNAL_SERVER_ERROR, "search_error"),
            MetadataError::MissingSlug => (StatusCode::BAD_REQUEST, "missing_slug"),
        }
    }
}
