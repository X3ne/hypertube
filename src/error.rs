use crate::auth::error::AuthError;
use crate::infrastructure::indexers::error::IndexerError;
use crate::infrastructure::metadata::error::MetadataError;
use crate::infrastructure::torrent::error::TorrentError as LibTorrentError;
use crate::torrents::error::TorrentError;
use crate::users::error::UserError;
use crate::{ApiErrorImpl, ErrorResponse};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(thiserror::Error, Debug, ApiErrorComponent)]
#[openapi_error(
    status(code = 403),
    status(code = 404),
    status(code = 405, description = "Invalid input"),
    status(code = 409)
)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Failed to parse uuid")]
    FailedToParseUuid(#[from] uuid::Error),
    #[error("Session error: {0}")]
    SessionError(#[from] actix_session::SessionInsertError),
    #[error("Session error: {0}")]
    SessionErrorGet(#[from] actix_session::SessionGetError),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error(transparent)]
    ValidationError(#[from] garde::error::Report),
    #[error(transparent)]
    MetadataError(#[from] MetadataError),
    #[error(transparent)]
    IndexerError(#[from] IndexerError),
    #[error(transparent)]
    LibTorrentError(#[from] LibTorrentError),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    TorrentError(#[from] TorrentError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    UserError(#[from] UserError),
}

impl ApiErrorImpl for ApiError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ApiError::BadRequest(..) => (StatusCode::BAD_REQUEST, "bad_request"),
            ApiError::FailedToParseUuid(..) => (StatusCode::BAD_REQUEST, "uuid_parse_error"),
            ApiError::SessionError(..) => (StatusCode::UNAUTHORIZED, "session_error"),
            ApiError::SessionErrorGet(..) => (StatusCode::UNAUTHORIZED, "session_error"),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error"),
            ApiError::ValidationError(..) => (StatusCode::BAD_REQUEST, "validation_error"),
            ApiError::MetadataError(err) => err.get_codes(),
            ApiError::IndexerError(err) => err.get_codes(),
            ApiError::LibTorrentError(err) => err.get_codes(),
            ApiError::DatabaseError(..) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            ApiError::TorrentError(err) => err.get_codes(),
            ApiError::AuthError(err) => err.get_codes(),
            ApiError::UserError(err) => err.get_codes(),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, code) = self.get_codes();
        let message = self.to_string();

        fn parse_errors(description: &str) -> HashMap<String, String> {
            let mut errors_map = HashMap::new();

            let re = Regex::new(r"(\w+):\s*([^\[]+)\[.*?\]").unwrap();

            for cap in re.captures_iter(description) {
                errors_map.insert(
                    cap[1].to_string(),
                    cap[2]
                        .to_string()
                        .replace("Validation error: ", "invalid_")
                        .trim()
                        .to_lowercase(),
                );
            }

            errors_map
        }

        let error_response = match self {
            ApiError::ValidationError(_) => {
                let description = self.to_string();
                ErrorResponse {
                    code,
                    message: "Validation error".to_string(),
                    details: None,
                    form_errors: Some(parse_errors(&description)),
                }
            }
            _ => ErrorResponse {
                code,
                message,
                details: None,
                form_errors: None,
            },
        };

        HttpResponse::build(status).json(error_response)
    }
}
