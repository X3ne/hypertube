use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use tracing_subscriber::fmt::Subscriber;

mod config;
mod error;
mod infrastructure;
mod server;
mod state;
mod utils;
pub use config::*;
pub use server::start_server;
pub use state::new_application_state;
pub use utils::telemetry::init_telemetry;

mod shows;
mod torrents;

pub fn init_service_logging() {
    let log_level = match std::env::var("RUST_LOG")
        .unwrap_or("info".to_string())
        .as_str()
    {
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    let subscriber = Subscriber::builder().with_max_level(log_level).finish();

    let _ = tracing::subscriber::set_global_default(subscriber).is_err();
    {
        eprintln!("Global tracing subscriber is already set; skipping telemetry initialization.");
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorDetails {
    pub message: String,
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse<'a> {
    pub code: &'a str,
    pub message: String,
    pub details: Option<Vec<ErrorDetails>>,
    pub form_errors: Option<HashMap<String, String>>,
}

impl ResponseError for ErrorResponse<'_> {}

impl Display for ErrorResponse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorResponse {{ code: {}, message: {}, details: {:?}, form_errors: {:?} }}",
            self.code, self.message, self.details, self.form_errors
        )
    }
}

pub trait ApiErrorImpl {
    fn get_codes(&self) -> (StatusCode, &str);
}

fn get_secret_key(secret_key: String) -> Key {
    let secret_key = secret_key.as_bytes();
    Key::derive_from(secret_key)
}
