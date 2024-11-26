use crate::error::ApiError;
use crate::infrastructure::indexers::indexer::{Indexer, Torrent};
use crate::infrastructure::metadata::error::MetadataError;
use crate::infrastructure::metadata::meta::Metadata;
use crate::infrastructure::metadata::provider::Provider;
use crate::infrastructure::metadata::providers::tvdb::config::TvdbSearchParam;
use crate::state::ApplicationState;
use actix_web::web;
use apistos::web::{get, resource, scope, ServiceConfig};
use apistos::{api_operation, ApiComponent};
use futures::future;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

pub fn config_shows(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/shows")
            .service(scope("/search").service(resource("").route(get().to(search_shows))))
            .service(
                scope("/tv")
                    // .service(resource("/trending").route(get().to(get_trending_shows)))
                    .service(
                        scope("/{slug}")
                            .service(resource("").route(get().to(get_tv)))
                            .service(resource("/torrent").route(get().to(get_tv_torrents))),
                    ),
            )
            .service(
                scope("/movies")
                    // .service(resource("/trending").route(get().to(get_trending_movies)))
                    .service(scope("/{slug}").service(resource("").route(get().to(get_movie)))),
            ),
    );
}

#[api_operation(
    tag = "shows",
    operation_id = "search_shows",
    summary = "Search for shows"
)]
#[instrument(skip(state))]
pub async fn search_shows(
    query: web::Query<TvdbSearchParam>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Vec<Metadata>>, ApiError> {
    let mut search_params = query.into_inner();

    // If no primary type is provided, search for both series and movies
    if search_params.primary_type.is_none() {
        search_params.primary_type = Some("series".to_string());

        let tv = state
            .metadata_provider()
            .search(search_params.clone())
            .await?;

        search_params.primary_type = Some("movie".to_string());

        let movies = state.metadata_provider().search(search_params).await?;

        let mut results = tv;
        results.extend(movies);

        return Ok(web::Json(results));
    }

    let results = state.metadata_provider().search(search_params).await?;

    Ok(web::Json(results))
}

#[api_operation(
    tag = "shows",
    operation_id = "get_tv",
    summary = "Get TV show metadata"
)]
#[instrument(skip(state))]
pub async fn get_tv(
    path: web::Path<String>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Metadata>, ApiError> {
    let slug = path.into_inner();

    let meta = state
        .metadata_provider()
        .get_tv_metadata(&slug, None)
        .await?;

    Ok(web::Json(meta))
}

#[derive(Debug, Clone, Deserialize, Serialize, ApiComponent, JsonSchema)]
struct Params {
    params: Option<String>,
}

#[api_operation(
    tag = "shows",
    operation_id = "get_tv_torrents",
    summary = "Get torrents for TV show"
)]
#[instrument(skip(state))]
pub async fn get_tv_torrents(
    path: web::Path<String>,
    query: web::Query<Params>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Vec<Torrent>>, ApiError> {
    let slug = path.into_inner();
    let params = query.into_inner();

    let meta = state
        .metadata_provider()
        .get_tv_metadata(&slug, None)
        .await?;

    let show_titles: Vec<&str> = match (&meta.title, &meta.aliases) {
        (Some(title), Some(aliases)) => aliases
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(title.as_str()))
            .collect(),
        (None, Some(aliases)) => aliases.iter().map(String::as_str).collect(),
        (Some(title), None) => vec![title.as_str()],
        _ => return Err(ApiError::MetadataError(MetadataError::MissingMetadata)),
    };

    let search_futures = show_titles.iter().map(|title| {
        let prowlarr_indexer = state.prowlarr_indexer();
        let value = params.params.clone();
        async move {
            prowlarr_indexer
                .search(format!("{} {}", title, value.unwrap_or_default()).as_str())
                .await
        }
    });

    let search_results = future::join_all(search_futures).await;

    let mut torrents_map = HashMap::new();
    for result in search_results {
        if let Ok(results) = result {
            for torrent in results {
                torrents_map.insert(torrent.name.clone(), torrent);
            }
        }
    }

    let torrents: Vec<Torrent> = torrents_map.into_values().collect();

    Ok(web::Json(torrents))
}

#[api_operation(
    tag = "shows",
    operation_id = "get_movie",
    summary = "Get movie metadata"
)]
#[instrument(skip(state))]
pub async fn get_movie(
    path: web::Path<String>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Metadata>, ApiError> {
    let slug = path.into_inner();

    let meta = state
        .metadata_provider()
        .get_tv_metadata(&slug, None)
        .await?;

    Ok(web::Json(meta))
}
