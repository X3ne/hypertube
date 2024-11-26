use crate::infrastructure::indexers::error::{IndexerError, Result};
use crate::infrastructure::indexers::indexer::{preprocess_name, Indexer, Source, Torrent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiTorrent {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Category")]
    pub category: Option<String>,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Size")]
    pub size: Option<String>,
    #[serde(rename = "Seeders")]
    pub seeders: Option<String>,
    #[serde(rename = "Leechers")]
    pub leechers: Option<String>,
    #[serde(rename = "Downloads")]
    pub downloads: Option<String>,
    #[serde(rename = "Torrent")]
    pub torrent: Option<String>,
    #[serde(rename = "Magnet")]
    pub magnet: Option<String>,
}

const API_URL: &str = "https://itorrentsearch.vercel.app/api";

pub struct GlobalIndexer {
    client: reqwest::Client,
}

impl GlobalIndexer {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl Indexer for GlobalIndexer {
    async fn search(&self, query: &str) -> Result<Vec<Torrent>> {
        let url = format!("{}/all/{}/1", API_URL, preprocess_name(query));
        let response = self.client.get(&url).send().await?;

        let torrents = response.json::<Vec<Vec<ApiTorrent>>>().await?;

        Ok(torrents
            .into_iter()
            .flatten()
            .map(Torrent::try_from)
            .filter_map(Result::ok)
            .collect())
    }
}

impl TryFrom<ApiTorrent> for Torrent {
    type Error = IndexerError;

    fn try_from(torrent: ApiTorrent) -> Result<Self> {
        let magnet = torrent.magnet.ok_or(IndexerError::MissingMagnet)?;

        Ok(Self {
            name: torrent.name,
            torrent: torrent.torrent,
            magnet,
            seeders: torrent.seeders.map(|s| s.parse().unwrap_or(0)).unwrap_or(0),
            leechers: torrent
                .leechers
                .map(|l| l.parse().unwrap_or(0))
                .unwrap_or(0),
            downloads: torrent
                .downloads
                .map(|d| d.parse().unwrap_or(0))
                .unwrap_or(0),
            size: torrent.size.unwrap_or("Unknown".to_string()),
            source: Source::from(torrent.url),
        })
    }
}
