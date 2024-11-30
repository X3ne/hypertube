use crate::error::ApiError;
use crate::state::ApplicationState;
use crate::torrents::error::TorrentError;
use crate::torrents::requests::AddTorrentWithMagnet;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use apistos::web::{post, resource, scope, ServiceConfig};
use garde::Validate;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse};
use std::sync::Arc;
use tracing::instrument;

pub fn config_torrent(cfg: &mut ServiceConfig) {
    cfg.service(scope("/torrents").service(resource("/magnet").route(post().to(add_torrent_with_magnet))));
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
