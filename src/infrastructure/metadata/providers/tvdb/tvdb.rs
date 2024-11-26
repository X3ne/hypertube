use crate::infrastructure::metadata::error::MetadataError;
use crate::infrastructure::metadata::error::Result;
use crate::infrastructure::metadata::meta::{Metadata, ShowType};
use crate::infrastructure::metadata::provider::Provider;
use crate::infrastructure::metadata::providers::tvdb::config::{TvdbConfig, TvdbSearchParam};
use tvdb4::apis::configuration::Configuration;
use tvdb4::apis::search_api::get_search_results;
use tvdb4::apis::{movies_api, series_api};
use tvdb4::models::{LoginPostRequest, MovieBaseRecord, SeriesBaseRecord, Translation};

enum TvOrMovie {
    Tv(SeriesBaseRecord),
    Movie(MovieBaseRecord),
}

pub struct TvdbProvider {
    config: Configuration,
}

fn filter_aliases(aliases: Option<Vec<tvdb4::models::Alias>>) -> Option<Vec<String>> {
    aliases.map(|alias_vec| {
        alias_vec
            .into_iter()
            .filter_map(|alias| {
                if let Some(language) = alias.language {
                    if language == "eng" || language == "fra" {
                        return alias.name;
                    }
                }
                None
            })
            .collect()
    })
}

impl TvdbProvider {
    pub async fn new(api_key: String) -> Result<Self> {
        let mut config = Configuration::new();
        let token = tvdb4::apis::login_api::login_post(&config, LoginPostRequest::new(api_key))
            .await
            .map_err(|_| MetadataError::AuthError)?
            .data
            .unwrap()
            .token;

        config.bearer_access_token = token;
        config.user_agent = Some("hypertube-rs".to_string());

        Ok(Self { config })
    }

    async fn get_tv_translation(&self, id: f32, language: &str) -> Result<Translation> {
        let data = series_api::get_series_translation(&self.config, id, language)
            .await
            .map_err(|_| MetadataError::FailedToGetMetadata)
            .map(|translations| translations.data)?;

        if let Some(data) = data {
            return Ok(*data);
        }

        Err(MetadataError::MissingMetadata)
    }

    async fn get_movie_translation(&self, id: f32, language: &str) -> Result<Translation> {
        let data = movies_api::get_movie_translation(&self.config, id, language)
            .await
            .map_err(|_| MetadataError::FailedToGetMetadata)
            .map(|translations| translations.data)?;

        if let Some(data) = data {
            return Ok(*data);
        }

        Err(MetadataError::MissingMetadata)
    }
}

#[async_trait::async_trait]
impl Provider<TvdbConfig, TvdbSearchParam> for TvdbProvider {
    async fn get_movie_metadata(&self, slug: &str, config: Option<TvdbConfig>) -> Result<Metadata> {
        let movie = movies_api::get_movie_base_by_slug(&self.config, slug)
            .await
            .map_err(|_| MetadataError::FailedToGetMetadata)?
            .data
            .ok_or(MetadataError::MissingMetadata)?;

        let translation = self
            .get_movie_translation(
                movie.id.unwrap() as f32,
                &config.unwrap_or_default().language,
            )
            .await?;

        let meta = Metadata::try_from_with_translation(TvOrMovie::Movie(*movie), translation)?;

        Ok(meta)
    }

    async fn get_tv_metadata(&self, slug: &str, config: Option<TvdbConfig>) -> Result<Metadata> {
        let show = series_api::get_series_base_by_slug(&self.config, slug)
            .await
            .map_err(|_| MetadataError::FailedToGetMetadata)?
            .data
            .ok_or(MetadataError::MissingMetadata)?;

        let translation = self
            .get_tv_translation(
                show.id.unwrap() as f32,
                &config.unwrap_or_default().language,
            )
            .await?;

        let meta = Metadata::try_from_with_translation(TvOrMovie::Tv(*show), translation)?;

        Ok(meta)
    }

    async fn search(&self, mut search: TvdbSearchParam) -> Result<Vec<Metadata>> {
        search.language.get_or_insert("eng".to_string());

        let results = get_search_results(
            &self.config,
            search.query.as_deref(),
            None,
            search.r#type.as_deref(),
            search.year,
            search.company.as_deref(),
            search.country.as_deref(),
            search.director.as_deref(),
            search.language.as_deref(),
            search.primary_type.as_deref(),
            search.network.as_deref(),
            search.remote_id.as_deref(),
            search.offset,
            search.limit,
        )
        .await
        .map_err(|_| MetadataError::SearchError)?
        .data
        .ok_or(MetadataError::SearchError)?;

        let results = results
            .iter()
            .map(|data| {
                let title = data.translations.as_ref().and_then(|translations| {
                    translations
                        .get(search.language.as_deref().unwrap())
                        .cloned()
                });
                let summary = data.overviews.as_ref().and_then(|overviews| {
                    overviews.get(search.language.as_deref().unwrap()).cloned()
                });

                let show_type = data
                    .r#type
                    .as_ref()
                    .and_then(|primary_type| match primary_type.as_str() {
                        "series" => Some(ShowType::Tv),
                        "movie" => Some(ShowType::Movie),
                        _ => None,
                    })
                    .unwrap_or(ShowType::Undefined);

                Metadata {
                    slug: data.slug.clone().unwrap_or("".to_string()),
                    title,
                    year: data.year.clone(),
                    rating: None,
                    poster: data.image_url.clone(),
                    backdrop: None,
                    show_type,
                    seasons: None,
                    aired_date: Default::default(),
                    summary,
                    aliases: data.aliases.clone(),
                }
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

impl Metadata {
    fn try_from_with_translation(value: TvOrMovie, translation: Translation) -> Result<Metadata> {
        match value {
            TvOrMovie::Tv(show) => {
                let slug = show.slug.ok_or(MetadataError::MissingSlug)?;

                let aliases = filter_aliases(show.aliases);

                Ok(Metadata {
                    slug,
                    title: translation.name,
                    year: show.year,
                    rating: show.score,
                    poster: show.image,
                    show_type: ShowType::Tv,
                    summary: translation.overview,
                    aliases,
                    // TODO: Implement this
                    backdrop: None,
                    seasons: Some(vec![]),
                    aired_date: Default::default(),
                })
            }
            TvOrMovie::Movie(movie) => {
                let slug = movie.slug.ok_or(MetadataError::MissingSlug)?;

                let aliases = filter_aliases(movie.aliases);

                Ok(Metadata {
                    slug,
                    title: translation.name,
                    year: movie.year,
                    rating: movie.score,
                    poster: movie.image,
                    show_type: ShowType::Movie,
                    seasons: None,
                    summary: translation.overview,
                    aliases,
                    // TODO: Implement this
                    backdrop: None,
                    aired_date: Default::default(),
                })
            }
        }
    }
}
