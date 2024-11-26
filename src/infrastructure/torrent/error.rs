use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

pub(super) type Result<T> = std::result::Result<T, TorrentError>;

#[derive(Debug, thiserror::Error)]
pub enum TorrentError {
    #[error("Invalid torrents hash")]
    InvalidHash,
    #[error("Torrent not found")]
    TorrentNotFound,
    #[error("Invalid lengths")]
    InvalidLengths,
    #[error("File not found")]
    FileNotFound,
}

impl ApiErrorImpl for TorrentError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            TorrentError::InvalidHash => (StatusCode::BAD_REQUEST, "invalid_hash"),
            TorrentError::TorrentNotFound => (StatusCode::NOT_FOUND, "torrent_not_found"),
            TorrentError::InvalidLengths => (StatusCode::INTERNAL_SERVER_ERROR, "invalid_lengths"),
            TorrentError::FileNotFound => (StatusCode::NOT_FOUND, "file_not_found"),
        }
    }
}
