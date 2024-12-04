use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, SqlitePool};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Schema error: {0}")]
    SchemaError(String),
}

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let database_url = database_url.replace("\"", "");
    log::info!("Connecting to database: {}", database_url);

    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        tracing::info!("Creating database {}", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => tracing::info!("Create db success"),
            Err(error) => tracing::error!("error: {}", error),
        }
    } else {
        tracing::info!("Database already exists");
    }

    let pool = SqlitePoolOptions::new()
        .min_connections(0)
        .max_connections(5)
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn make_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");

    sqlx::migrate!("./migrations").run(pool).await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
