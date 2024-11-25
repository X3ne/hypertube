use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("Failed to acquire stream")]
    FailedToAcquireStream,
}

impl ApiErrorImpl for StreamError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            StreamError::FailedToAcquireStream => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed_to_acquire_stream",
            ),
        }
    }
}
