use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum TranscodeError {
    #[error("Failed to acquire stream")]
    FailedToAcquireStream,
}

impl ApiErrorImpl for TranscodeError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            TranscodeError::FailedToAcquireStream => (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_acquire_stream"),
        }
    }
}
