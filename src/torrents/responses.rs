use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Debug, ApiComponent, JsonSchema)]
pub enum TorrentStatsState {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Default, Debug, ApiComponent, JsonSchema)]
pub struct Speed {
    pub mbps: f64,
}

#[derive(Serialize, Default, Debug, ApiComponent, JsonSchema)]
pub struct LiveStats {
    pub average_piece_download_time: Option<Duration>,
    pub download_speed: Speed,
    pub upload_speed: Speed,
    pub time_remaining: Option<String>,
}

#[derive(Serialize, Debug, ApiComponent, JsonSchema)]
pub struct TorrentStats {
    pub state: TorrentStatsState,
    pub file_progress: Vec<u64>,
    pub error: Option<String>,
    pub progress_bytes: u64,
    pub uploaded_bytes: u64,
    pub total_bytes: u64,
    pub finished: bool,
    pub live: Option<LiveStats>,
}

impl From<librqbit::TorrentStats> for TorrentStats {
    fn from(value: librqbit::TorrentStats) -> Self {
        Self {
            state: match value.state {
                librqbit::TorrentStatsState::Initializing => TorrentStatsState::Initializing,
                librqbit::TorrentStatsState::Live => TorrentStatsState::Live,
                librqbit::TorrentStatsState::Paused => TorrentStatsState::Paused,
                librqbit::TorrentStatsState::Error => TorrentStatsState::Error,
            },
            file_progress: value.file_progress,
            error: value.error,
            progress_bytes: value.progress_bytes,
            uploaded_bytes: value.uploaded_bytes,
            total_bytes: value.total_bytes,
            finished: value.finished,
            live: value.live.map(|live| LiveStats {
                average_piece_download_time: live.average_piece_download_time,
                download_speed: Speed {
                    mbps: live.download_speed.mbps,
                },
                upload_speed: Speed {
                    mbps: live.upload_speed.mbps,
                },
                time_remaining: live.time_remaining.map(|duration| duration.to_string()),
            }),
        }
    }
}
