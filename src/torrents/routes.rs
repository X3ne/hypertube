use crate::error::ApiError;
use crate::infrastructure::torrent::get_torrent_handle;
use crate::state::ApplicationState;
use crate::torrents::error::TorrentError;
use crate::torrents::requests::AddTorrentWithMagnet;
use crate::torrents::responses::TorrentStats;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use apistos::web::{get, post, resource, scope, ServiceConfig};
use garde::Validate;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse};
use std::sync::Arc;
use tracing::instrument;

pub fn config_torrent(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/torrents")
            .service(resource("/magnet").route(post().to(add_torrent_with_magnet)))
            .service(
                scope("/{torrent_hash}").service(resource("/stats").route(get().to(get_stats))), // .service(resource("/pieces").route(get().to(get_pieces_info))),
            ),
    );
}

#[api_operation(
    tag = "torrents",
    operation_id = "add_torrent_with_magnet",
    summary = "Add a new torrents with a magnet link"
)]
#[instrument(skip(state))]
pub async fn add_torrent_with_magnet(
    body: web::Json<AddTorrentWithMagnet>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<NoContent, ApiError> {
    let body = body.into_inner();
    body.validate()?;

    match state
        .manager()
        .add_torrent(
            AddTorrent::from_url(&body.magnet),
            Some(AddTorrentOptions {
                overwrite: true,
                ..Default::default()
            }),
        )
        .await
    {
        Ok(AddTorrentResponse::Added(_, _)) => {
            tracing::info!("Torrent added successfully");
            Ok(NoContent)
        }
        Ok(AddTorrentResponse::AlreadyManaged(..)) => Err(TorrentError::TorrentAlreadyAdded.into()),
        Err(e) => {
            tracing::error!("Error adding torrents: {:?}", e);
            Err(TorrentError::AddTorrentError.into())
        }
        _ => Err(TorrentError::AddTorrentError.into()),
    }
}

#[api_operation(
    tag = "torrents",
    operation_id = "get_stats",
    summary = "Get the stats of a torrents"
)]
#[instrument(skip(state))]
pub async fn get_stats(
    path: web::Path<String>,
    state: web::Data<Arc<ApplicationState>>,
) -> Result<web::Json<TorrentStats>, ApiError> {
    let hash = path.into_inner();

    let handle = get_torrent_handle(state.manager(), &hash)?;

    let stats = handle.stats();
    Ok(web::Json(stats.into()))
}
