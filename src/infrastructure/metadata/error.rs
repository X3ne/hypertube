use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

pub(super) type Result<T> = std::result::Result<T, MetadataError>;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Failed to search show")]
    SearchError,
    #[error("Http error: {0}")]
    HttpError(#[from] reqwest::Error),
}

impl ApiErrorImpl for MetadataError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            MetadataError::SearchError => (StatusCode::INTERNAL_SERVER_ERROR, "search_error"),
            MetadataError::HttpError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "http_error"),
        }
    }
}
