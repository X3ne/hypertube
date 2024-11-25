use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum TorrentError {
    #[error("Invalid torrents hash")]
    InvalidHash,
    #[error("Torrent not found")]
    TorrentNotFound,
}

impl ApiErrorImpl for TorrentError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            TorrentError::InvalidHash => (StatusCode::BAD_REQUEST, "invalid_hash"),
            TorrentError::TorrentNotFound => (StatusCode::NOT_FOUND, "torrent_not_found"),
        }
    }
}
