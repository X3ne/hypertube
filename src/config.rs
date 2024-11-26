use dotenvy::dotenv;
use serde::Deserialize;
use std::path::PathBuf;

#[cfg(test)]
fn load_env() {
    dotenvy::from_filename(".env.test").ok();
}

#[cfg(not(test))]
fn load_env() {
    dotenv().ok();
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub api_version: String,
    pub host: String,
    pub port: u16,
    pub origins: Vec<String>,
    pub telemetry_collector_endpoint: Option<String>,
    pub download_dir: PathBuf,
    pub tvdb_api_key: String,
    pub prowlarr_api_key: String,
    pub prowlarr_api_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .list_separator(",")
                    .with_list_parse_key("origins")
                    .try_parsing(true),
            )
            .set_default("api_version", env!("CARGO_PKG_VERSION"))
            .unwrap()
            .set_default("host", "0.0.0.0")
            .unwrap()
            .set_default("port", 3000)
            .unwrap()
            .build()?;

        let cfg: Config = config.try_deserialize()?;

        Ok(cfg)
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct CookieConfig {
    pub session_secret: String,
    pub session_ttl: i64,
    pub same_site: String,
    pub secure: bool,
    pub http_only: bool,
}

impl CookieConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("cookie").try_parsing(true))
            .set_default("same_site", "strict") // strict, lax, none
            .unwrap()
            .set_default("secure", true)
            .unwrap()
            .set_default("http_only", true)
            .unwrap()
            .build()?;

        let cfg: CookieConfig = config.try_deserialize()?;

        Ok(cfg)
    }
}
