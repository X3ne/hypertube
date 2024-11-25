use std::sync::Arc;

use crate::config::Config;
use crate::error::ApiError;
use crate::get_secret_key;
use crate::state::ApplicationState;
use crate::utils::cors::default_cors;
use actix_session::config::PersistentSession;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::cookie::SameSite;
use actix_web::web::Json;
use actix_web::{web, App, HttpServer};
use apistos::web::{get, resource};
use apistos::{
    api_operation,
    app::{BuildConfig, OpenApiWrapper},
    info::Info,
    spec::Spec,
    web::scope,
    ApiComponent, ScalarConfig,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tracing_actix_web::TracingLogger;

pub struct ActixServer {
    pub handle: JoinHandle<()>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ServerHealth {
    pub status: String,
}

#[api_operation(tag = "health", operation_id = "health")]
async fn health() -> Result<Json<ServerHealth>, ApiError> {
    Ok(Json(ServerHealth {
        status: "ok".to_string(),
    }))
}

pub fn start_server(
    state: Arc<ApplicationState>,
    cfg: Arc<Config>,
    host: String,
    port: u16,
) -> Result<ActixServer, Box<dyn std::error::Error>> {
    let cookie_cfg =
        crate::config::CookieConfig::from_env().expect("Failed to load cookie configuration");

    let server = HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "Hypertube API".to_string(),
                version: "0.1.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        App::new()
            .document(spec)
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    get_secret_key(cookie_cfg.session_secret.clone()),
                )
                .cookie_secure(cookie_cfg.secure)
                .cookie_http_only(cookie_cfg.http_only)
                .cookie_same_site(match cookie_cfg.same_site.as_str() {
                    "strict" => SameSite::Strict,
                    "lax" => SameSite::Lax,
                    "none" => SameSite::None,
                    _ => SameSite::Strict,
                })
                .cookie_name("session".to_string())
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::new(cookie_cfg.session_ttl, 0)),
                )
                .build(),
            )
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(state.clone()))
            .app_data(
                web::FormConfig::default()
                    .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::PathConfig::default()
                    .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .configure(|app| {
                app.service(
                    scope("/v1")
                        .wrap(default_cors(cfg.origins.clone()))
                        .service(resource("").route(get().to(health)))
                        .configure(|cfg| {
                            crate::torrents::config_torrent(cfg);
                        }),
                );
            })
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run();

    let server_handle = tokio::spawn(async move {
        log::info!("Starting server on {}:{}", host, port);
        if let Err(e) = server.await {
            log::warn!("Server error: {}", e);
        }
    });

    Ok(ActixServer {
        handle: server_handle,
    })
}
