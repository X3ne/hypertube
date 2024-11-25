use crate::config::Config;
use librqbit::{Session, SessionOptions, SessionPersistenceConfig};
use std::sync::Arc;

pub struct ApplicationState {
    torrent_manager: Arc<Session>,
}

pub async fn new_application_state(cfg: Config) -> ApplicationState {
    let output_dir = "./downloads";
    let manager = Session::new_with_opts(
        output_dir.into(),
        SessionOptions {
            fastresume: true,
            persistence: Some(SessionPersistenceConfig::Json {
                folder: Some(output_dir.into()),
            }),

            ..Default::default()
        },
    )
    .await
    .expect("Failed to create torrents manager");

    ApplicationState {
        torrent_manager: manager,
    }
}

impl ApplicationState {
    pub fn manager(&self) -> &Arc<Session> {
        &self.torrent_manager
    }
}
