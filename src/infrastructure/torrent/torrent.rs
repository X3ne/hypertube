use super::error::Result;
use crate::infrastructure::torrent::error::TorrentError;
use librqbit::api::TorrentIdOrHash;
use librqbit::{ManagedTorrent, Session};
use librqbit_core::Id20;
use std::str::FromStr;
use std::sync::Arc;

pub fn get_torrent_handle<S: AsRef<str>>(
    manager: &Arc<Session>,
    hash: S,
) -> Result<Arc<ManagedTorrent>> {
    let hash_ref = hash.as_ref();

    let id = Id20::from_str(hash_ref).map_err(|_| TorrentError::InvalidHash)?;

    match manager.get(TorrentIdOrHash::Hash(id)) {
        Some(handle) => {
            tracing::debug!("Found torrents: {:?}", handle.info_hash());
            Ok(handle)
        }
        None => Err(TorrentError::TorrentNotFound),
    }
}
