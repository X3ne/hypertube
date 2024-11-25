use crate::error::ApiError;
use crate::infrastructure::torrent::get_torrent_handle;
use crate::state::ApplicationState;
use crate::torrents::create_torrent_playlist_items;
use crate::torrents::error::TorrentError;
use crate::torrents::requests::AddTorrentWithMagnet;
use crate::torrents::responses::TorrentStats;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpRequest, HttpResponse};
use apistos::actix::NoContent;
use apistos::api_operation;
use apistos::web::{get, post, resource, scope, ServiceConfig};
use garde::Validate;
use itertools::Itertools;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse};
use std::io::SeekFrom;
use std::sync::Arc;
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;
use tracing::instrument;

pub fn config_torrent(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/torrents")
            .service(resource("/magnet").route(post().to(add_torrent_with_magnet)))
            .service(
                scope("/{torrent_hash}")
                    .service(resource("/stats").route(get().to(get_stats)))
                    .service(
                        scope("/stream")
                            .service(resource("/{file_id}").route(get().to(stream_file)))
                            .service(resource("/{file_id}/{filename}").route(get().to(stream_file)))
                            .service(resource("/playlist").route(get().to(create_playlist))),
                    ),
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

#[api_operation(tag = "stream", operation_id = "stream", summary = "Stream a torrents")]
#[instrument(skip(state))]
pub async fn stream_file(
    params: web::Path<(String, usize)>,
    state: web::Data<Arc<ApplicationState>>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (hash, file_id) = params.into_inner();
    let headers = req.headers();

    let range_header = headers.get(header::RANGE);
    let mut response = match range_header {
        Some(_) => HttpResponse::PartialContent(),
        None => HttpResponse::Ok(),
    };

    let handle = get_torrent_handle(state.manager(), &hash)?;

    let mut stream = handle.stream(file_id).map_err(|e| {
        tracing::error!("Error streaming torrents: {:?}", e);
        TorrentError::FailedToAcquireStream
    })?;

    tracing::trace!(torrent=%hash, file_id=file_id, range=?range_header, "request for HTTP stream");

    if let Some(range) = range_header {
        let offset: Option<u64> = range
            .to_str()
            .ok()
            .and_then(|s| s.strip_prefix("bytes="))
            .and_then(|s| s.strip_suffix('-'))
            .and_then(|s| s.parse().ok());
        if let Some(offset) = offset {
            stream
                .seek(SeekFrom::Start(offset))
                .await
                .map_err(|_| ApiError::InternalServerError)?;

            response.insert_header((
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&format!("{}", stream.len() - stream.position()))
                    .map_err(|_| ApiError::InternalServerError)?,
            ));
            response.insert_header((
                header::CONTENT_RANGE,
                HeaderValue::from_str(&format!(
                    "bytes {}-{}/{}",
                    stream.position(),
                    stream.len().saturating_sub(1),
                    stream.len()
                ))
                .map_err(|_| ApiError::InternalServerError)?,
            ));
        }
    } else {
        response.insert_header((
            header::CONTENT_LENGTH,
            HeaderValue::from_str(&format!("{}", stream.len()))
                .map_err(|_| ApiError::InternalServerError)?,
        ));
    }

    let response_stream = ReaderStream::with_capacity(stream, 65536);

    Ok(response
        .insert_header((header::CONTENT_TYPE, "video/mkv"))
        .streaming(response_stream))
}

#[api_operation(
    tag = "stream",
    operation_id = "create_playlist",
    summary = "Create M3U8 playlist"
)]
#[instrument(skip(state))]
pub async fn create_playlist(
    path: web::Path<String>,
    state: web::Data<Arc<ApplicationState>>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let hash = path.into_inner();

    let host = req.connection_info().host().to_string();

    let handle = get_torrent_handle(state.manager(), &hash)?;

    let playlist_items = create_torrent_playlist_items(&handle)?;

    let body = playlist_items
        .into_iter()
        .map(move |(file_idx, filename)| (hash.clone(), file_idx, filename))
        .into_iter()
        .map(|(torrent_hash, file_id, filename)| {
            format!("http://{host}/v1/torrents/{torrent_hash}/stream/{file_id}/{filename}")
        })
        .join("\r\n");

    Ok(HttpResponse::Ok()
        .content_type("application/mpegurl; charset=utf-8")
        .insert_header((
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"hypertube-playlist.m3u8\"".to_string(),
        ))
        .body(format!("#EXTM3U\r\n{body}")))
}
