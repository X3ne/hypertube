use crate::error::ApiError;
use crate::infrastructure::indexers::indexer::{Indexer, Torrent};
use crate::infrastructure::metadata::error::MetadataError;
use crate::infrastructure::metadata::models::{Movie, SearchParams, SearchResults, TvSeason, Video, TV};
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
                    .service(resource("/trending").route(get().to(get_tv_trending)))
                    .service(
                        scope("/{id}")
                            .service(resource("").route(get().to(get_tv)))
                            .service(resource("/season/{season_number}").route(get().to(get_tv_season)))
                            .service(resource("/torrent").route(get().to(get_tv_torrents))),
                    ),
            )
            .service(
                scope("/movies")
                    .service(resource("/trending").route(get().to(get_movie_trending)))
                    .service(scope("/{id}").service(resource("").route(get().to(get_movie)))),
            ),
    );
}

#[api_operation(tag = "shows", operation_id = "search_shows", summary = "Search for shows")]
#[instrument(skip(state))]
pub async fn search_shows(
    query: web::Query<SearchParams>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<SearchResults>, ApiError> {
    let search_params = query.into_inner();

    let results = state.metadata_provider().search(search_params).await?;

    Ok(web::Json(results))
}

#[api_operation(
    tag = "shows",
    operation_id = "get_tv_trending",
    summary = "Get trending TV shows this week"
)]
#[instrument(skip(state))]
pub async fn get_tv_trending(state: web::Data<Arc<ApplicationState>>) -> Result<web::Json<Vec<TV>>, ApiError> {
    let meta = state.metadata_provider().get_tv_trending(None).await?;

    Ok(web::Json(meta))
}

#[api_operation(tag = "shows", operation_id = "get_tv", summary = "Get TV show metadata")]
#[instrument(skip(state))]
pub async fn get_tv(path: web::Path<u32>, state: web::Data<Arc<ApplicationState>>) -> Result<web::Json<TV>, ApiError> {
    let id = path.into_inner();

    let meta = state.metadata_provider().get_tv_metadata(id, None).await?;

    Ok(web::Json(meta))
}

#[api_operation(
    tag = "shows",
    operation_id = "get_tv_season",
    summary = "Get TV show season metadata"
)]
#[instrument(skip(state))]
pub async fn get_tv_season(
    path: web::Path<(u32, u32)>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<TvSeason>, ApiError> {
    let (id, season_number) = path.into_inner();

    let meta = state.metadata_provider().get_tv_season(id, season_number, None).await?;

    Ok(web::Json(meta))
}

#[api_operation(
    tag = "shows",
    operation_id = "get_movie_trending",
    summary = "Get trending movies this week"
)]
#[instrument(skip(state))]
pub async fn get_movie_trending(state: web::Data<Arc<ApplicationState>>) -> Result<web::Json<Vec<Movie>>, ApiError> {
    let meta = state.metadata_provider().get_movie_trending(None).await?;

    Ok(web::Json(meta))
}

#[api_operation(tag = "shows", operation_id = "get_movie", summary = "Get movie metadata")]
#[instrument(skip(state))]
pub async fn get_movie(
    path: web::Path<u32>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Movie>, ApiError> {
    let id = path.into_inner();

    let meta = state.metadata_provider().get_movie_metadata(id, None).await?;

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
    path: web::Path<u32>,
    query: web::Query<Params>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<Vec<Torrent>>, ApiError> {
    let id = path.into_inner();
    let params = query.into_inner();

    let meta = state.metadata_provider().get_tv_metadata(id, None).await?;

    let show_titles: Vec<&str> = match (&meta.name, &meta.alternative_titles) {
        (title, Some(aliases)) => {
            let mut titles = vec![title.as_str()];
            titles.extend(aliases.results.iter().filter_map(|title| {
                let iso_code = title.iso_3166_1.as_str();
                let title_type = title.r#type.clone();
                if iso_code == "GB"
                    || iso_code == "US"
                    || (iso_code == "JP" && (title_type == "romaji" || title_type == "romanization"))
                {
                    Some(title.title.as_str())
                } else {
                    None
                }
            }));
            titles
        }
        (title, None) => vec![title.as_str()],
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
