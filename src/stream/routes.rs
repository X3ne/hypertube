use crate::error::ApiError;
use crate::infrastructure::torrent::get_torrent_handle;
use crate::state::ApplicationState;
use crate::stream::error::StreamError;
use crate::stream::torrent_playlist_items;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpRequest, HttpResponse};
use apistos::api_operation;
use apistos::web::{get, resource, scope, ServiceConfig};
use itertools::Itertools;
use std::io::SeekFrom;
use std::sync::Arc;
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;
use tracing::instrument;

pub fn config_stream(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/stream")
            .service(resource("/{hash}/{file_id}/{filename}").route(get().to(stream_file)))
            .service(resource("/{hash}/playlist").route(get().to(create_playlist))),
    );
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
        StreamError::FailedToAcquireStream
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

    let playlist_items = torrent_playlist_items(&handle)?;

    let body = playlist_items
        .into_iter()
        .map(move |(file_idx, filename)| (hash.clone(), file_idx, filename))
        .into_iter()
        .map(|(torrent_hash, file_id, filename)| {
            format!("http://{host}/v1/stream/{torrent_hash}/{file_id}/{filename}")
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
