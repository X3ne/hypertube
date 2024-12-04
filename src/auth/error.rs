use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Failed to create token")]
    FailedToCreateToken,
    #[error("Failed to validate token")]
    FailedToValidateToken,
    #[error("Failed to request token")]
    FailedToRequestToken,
    #[error("Failed to request users info")]
    FailedToRequestUserInfo,
    #[error("Invalid provider")]
    InvalidProvider,
}

impl ApiErrorImpl for AuthError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
            AuthError::FailedToCreateToken => (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_create_token"),
            AuthError::FailedToValidateToken => (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_validate_token"),
            AuthError::FailedToRequestToken => (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_request_token"),
            AuthError::FailedToRequestUserInfo => (StatusCode::INTERNAL_SERVER_ERROR, "failed_to_request_user_info"),
            AuthError::InvalidProvider => (StatusCode::INTERNAL_SERVER_ERROR, "invalid_provider"),
        }
    }
}
