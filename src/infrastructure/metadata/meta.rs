use apistos::ApiComponent;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, ApiComponent, JsonSchema)]
pub enum ShowType {
    Movie,
    Tv,
    Undefined,
}

#[derive(Debug, Serialize, ApiComponent, JsonSchema)]
pub struct EpisodeMeta {
    pub title: String,
    pub year: i32,
    pub rating: f32,
    pub summary: String,
    pub poster: String,
}

#[derive(Debug, Serialize, ApiComponent, JsonSchema)]
pub struct SeasonMeta {
    pub title: String,
    pub year: i32,
    pub rating: f32,
    pub summary: String,
    pub poster: String,
    pub episodes: Vec<EpisodeMeta>,
}

#[derive(Debug, Serialize, ApiComponent, JsonSchema)]
pub struct Metadata {
    pub slug: String,
    pub title: Option<String>,
    pub year: Option<String>,
    pub rating: Option<f64>,
    pub summary: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub show_type: ShowType,
    pub seasons: Option<Vec<SeasonMeta>>,
    pub aired_date: Option<NaiveDateTime>,
}
