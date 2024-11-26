use crate::infrastructure::metadata::provider::{ProviderConfig, ProviderSearchParam};
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

pub struct TvdbConfig {
    pub language: String,
}

impl ProviderConfig for TvdbConfig {}

impl Default for TvdbConfig {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
        }
    }
}

impl TvdbConfig {
    pub fn new(language: String) -> Self {
        Self { language }
    }
}

#[derive(Debug, Clone, Deserialize, ApiComponent, JsonSchema)]
pub struct TvdbSearchParam {
    pub query: Option<String>,
    pub r#type: Option<String>,
    pub year: Option<f32>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub director: Option<String>,
    pub language: Option<String>,
    pub primary_type: Option<String>,
    pub network: Option<String>,
    pub remote_id: Option<String>,
    pub offset: Option<f32>,
    pub limit: Option<f32>,
}

impl<'a> ProviderSearchParam for TvdbSearchParam {}
