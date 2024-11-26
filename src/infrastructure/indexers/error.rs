use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

pub(super) type Result<T> = std::result::Result<T, IndexerError>;

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("Error requesting data from indexer")]
    HttpError(#[from] reqwest::Error),
    #[error("Missing magnet link")]
    MissingMagnet,
}

impl ApiErrorImpl for IndexerError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            IndexerError::HttpError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "http_error"),
            IndexerError::MissingMagnet => (StatusCode::INTERNAL_SERVER_ERROR, "missing_magnet"),
        }
    }
}
