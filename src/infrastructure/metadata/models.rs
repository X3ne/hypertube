use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub enum ShowType {
    #[default]
    Movie,
    TV,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Results<T: JsonSchema> {
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Genre {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Video {
    pub iso_639_1: String,
    pub iso_3166_1: String,
    pub name: String,
    pub key: String,
    pub site: String,
    pub size: u32,
    #[serde(rename = "type")]
    pub r#type: String,
    pub official: bool,
    pub published_at: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct AlternativeTitle {
    pub iso_3166_1: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Cast {
    pub id: u32,
    pub cast_id: u32,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u8,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct TVCast {
    pub id: u32,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u32,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct TVCreator {
    pub id: u32,
    pub name: String,
    pub gender: Option<u8>,
    pub profile_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Crew {
    pub department: String,
    pub gender: Option<u8>,
    pub id: u32,
    pub job: String,
    pub name: String,
    pub profile_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct TVCredits {
    pub cast: Vec<TVCast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct LastEpisode {
    pub air_date: String,
    pub episode_number: u32,
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub season_number: u32,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct ProductionCompany {
    pub id: u32,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Network {
    pub id: u32,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u32,
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub season_number: u32,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Movie {
    pub id: u32,
    pub imdb_id: Option<u32>,
    pub title: String,
    pub tagline: Option<String>,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub runtime: Option<u32>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: Option<u64>,
    pub adult: bool,
    pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
    #[serde(skip_deserializing)]
    pub show_type: ShowType,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct Episode {
    pub air_date: String,
    pub episode_number: i64,
    pub episode_type: String,
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub production_code: String,
    pub runtime: Option<i64>,
    pub season_number: i64,
    pub show_id: i64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: i64,
    pub crew: Vec<Crew>,
    #[serde(default)]
    pub guest_stars: Vec<TVCast>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct TvSeason {
    pub air_date: String,
    #[serde(default)]
    pub episodes: Vec<Episode>,
    pub name: String,
    pub overview: String,
    pub id: i64,
    pub poster_path: String,
    pub season_number: i64,
    pub vote_average: f64,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct TV {
    pub id: u32,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub created_by: Vec<TVCreator>,
    #[serde(default)]
    pub episode_run_time: Vec<u16>,
    pub first_air_date: String,
    #[serde(default)]
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub in_production: bool,
    #[serde(default)]
    pub languages: Vec<String>,
    pub last_air_date: Option<String>,
    pub last_episode_to_air: Option<LastEpisode>,
    pub name: String,
    #[serde(default)]
    pub networks: Vec<Network>,
    #[serde(default)]
    pub number_of_episodes: u32,
    #[serde(default)]
    pub number_of_seasons: u32,
    #[serde(default)]
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub production_companies: Vec<ProductionCompany>,
    #[serde(default)]
    pub seasons: Vec<Season>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
    pub videos: Option<Results<Video>>,
    pub credits: Option<TVCredits>,
    pub alternative_titles: Option<Results<AlternativeTitle>>,
    #[serde(skip_deserializing)]
    pub show_type: ShowType,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct FindMovie {
    pub id: u32,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: Option<f64>,
    pub adult: bool,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct FindTV {
    pub id: u32,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub first_air_date: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: Option<f64>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u32>,
    pub adult: bool,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct PaginatedResult<T: JsonSchema> {
    pub page: u16,
    pub total_results: u16,
    pub total_pages: u16,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct SearchParams {
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct SearchResults {
    pub tv: Vec<FindTV>,
    pub movies: Vec<FindMovie>,
}
