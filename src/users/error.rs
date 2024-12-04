use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error")]
    DatabaseError,
}

impl ApiErrorImpl for UserError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            UserError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found"),
            UserError::UserAlreadyExists => (StatusCode::CONFLICT, "user_already_exists"),
            UserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => UserError::UserNotFound,
            sqlx::Error::Database(err) => {
                tracing::error!("Database error: {}", err);
                if let Some(constraint) = err.constraint() {
                    return match constraint {
                        "user_email" | "user_username" => UserError::UserAlreadyExists,
                        _ => UserError::DatabaseError,
                    };
                }
                UserError::DatabaseError
            }
            _ => UserError::DatabaseError,
        }
    }
}
