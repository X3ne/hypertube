use crate::error::ApiError;
use crate::infrastructure::metadata::meta::Metadata;
use crate::infrastructure::metadata::provider::Provider;
use crate::infrastructure::metadata::providers::tvdb::config::TvdbSearchParam;
use crate::state::ApplicationState;
use actix_web::web;
use apistos::api_operation;
use apistos::web::{get, resource, scope, ServiceConfig};
use std::sync::Arc;
use tracing::instrument;

pub fn config_shows(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/shows")
            .service(scope("/search").service(resource("").route(get().to(search_shows))))
            .service(
                scope("/tv")
                    // .service(resource("/trending").route(get().to(get_trending_shows)))
                    .service(scope("/{slug}").service(resource("").route(get().to(get_tv)))),
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
