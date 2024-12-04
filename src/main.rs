use hypertube::{database, init_service_logging, init_telemetry, new_application_state, start_server, Config};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::from_env().expect("Failed to load configuration");

    match cfg.telemetry_collector_endpoint {
        Some(ref endpoint) => init_telemetry(endpoint),
        None => init_service_logging(),
    }

    tracing::debug!("Configuration: {:?}", cfg);

    let pool = database::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    database::make_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let state = Arc::new(new_application_state(cfg.clone()).await);

    let port = cfg.port;
    let host = cfg.host.clone();

    let server = start_server(state, Arc::new(cfg), host, port, pool).expect("Failed to start server");

    server.handle.await?;

    Ok(())
}
