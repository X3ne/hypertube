use crate::config::Config;
use crate::infrastructure::indexers::global::GlobalIndexer;
use crate::infrastructure::indexers::prowlarr::ProwlarrIndexer;
use crate::infrastructure::metadata::tmdb::TmdbProvider;
use librqbit::{Session, SessionOptions, SessionPersistenceConfig};
use std::sync::Arc;

pub struct ApplicationState {
    torrent_manager: Arc<Session>,
    metadata_provider: Arc<TmdbProvider>,
    global_indexer: Arc<GlobalIndexer>,
    prowlarr_indexer: Arc<ProwlarrIndexer>,
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

    let provider = TmdbProvider::new(cfg.tmdb_api_key);

    let global_indexer = GlobalIndexer::new();
    let prowlarr_indexer = ProwlarrIndexer::new(cfg.prowlarr_api_url, cfg.prowlarr_api_key);

    ApplicationState {
        torrent_manager: manager,
        metadata_provider: Arc::new(provider),
        global_indexer: Arc::new(global_indexer),
        prowlarr_indexer: Arc::new(prowlarr_indexer),
    }
}

impl ApplicationState {
    pub fn manager(&self) -> &Arc<Session> {
        &self.torrent_manager
    }

    pub fn metadata_provider(&self) -> &Arc<TmdbProvider> {
        &self.metadata_provider
    }

    pub fn global_indexer(&self) -> &Arc<GlobalIndexer> {
        &self.global_indexer
    }

    pub fn prowlarr_indexer(&self) -> &Arc<ProwlarrIndexer> {
        &self.prowlarr_indexer
    }
}
