mod requests;
mod responses;
mod routes;

use crate::error::ApiError;
use librqbit::ManagedTorrent;
pub use routes::config_torrent;

pub mod error;

fn create_torrent_playlist_items(
    handle: &ManagedTorrent,
) -> Result<Vec<(usize, String)>, ApiError> {
    let mut playlist_items = handle
        .shared()
        .info
        .iter_file_details(
            &librqbit_core::lengths::Lengths::from_torrent(&handle.shared().info)
                .map_err(|_| ApiError::InternalServerError)?,
        )
        .map_err(|_| ApiError::InternalServerError)?
        .enumerate()
        .filter_map(|(file_idx, file_details)| {
            let filename = file_details.filename.to_vec().ok()?.join("/");
            let is_playable = mime_guess::from_path(&filename)
                .first()
                .map(|mime| mime.type_() == mime_guess::mime::VIDEO)
                .unwrap_or(false);
            if is_playable {
                let filename = urlencoding::encode(&filename);
                Some((file_idx, filename.into_owned()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    playlist_items.sort_by(|left, right| left.1.cmp(&right.1));
    Ok(playlist_items)
}
