use crate::ApiErrorImpl;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum TorrentError {
    #[error("Error adding torrents")]
    AddTorrentError,
    #[error("Torrent already added")]
    TorrentAlreadyAdded,
    #[error("Failed to initialize torrents")]
    TorrentInitializationFailed,
    #[error("Failed to acquire stream")]
    FailedToAcquireStream,
}

impl ApiErrorImpl for TorrentError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            TorrentError::AddTorrentError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "add_torrent_error")
            }
            TorrentError::TorrentAlreadyAdded => (StatusCode::CONFLICT, "torrent_already_added"),
            TorrentError::TorrentInitializationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "torrent_initialization_failed",
            ),
            TorrentError::FailedToAcquireStream => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed_to_acquire_stream",
            ),
        }
    }
}
