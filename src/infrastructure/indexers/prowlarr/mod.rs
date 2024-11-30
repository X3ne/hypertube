use crate::infrastructure::indexers::error::{IndexerError, Result};
use crate::infrastructure::indexers::indexer::{Indexer, Source, Torrent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ProwlarrTorrent {
    pub guid: String,
    pub size: i64,
    pub indexer: String,
    pub title: String,
    #[serde(rename = "magnetUrl")]
    pub magnet_url: String,
    pub seeders: i64,
    pub leechers: i64,
}

pub struct ProwlarrIndexer {
    client: reqwest::Client,
    api_url: String,
    api_key: String,
}

impl ProwlarrIndexer {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url,
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Indexer for ProwlarrIndexer {
    async fn search(&self, query: &str) -> Result<Vec<Torrent>> {
        let response = self
            .client
            .get(&format!("{}/search", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("query", query), ("type", "search"), ("limit", "100")])
            .send()
            .await?;

        if response.status().is_success() {
            let results: Vec<ProwlarrTorrent> = response.json().await?;

            Ok(results
                .into_iter()
                .map(|torrent| Torrent::try_from(torrent))
                .collect::<Result<Vec<Torrent>>>()?)
        } else {
            Err(IndexerError::HttpError(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )))
        }
    }
}

impl TryFrom<ProwlarrTorrent> for Torrent {
    type Error = IndexerError;

    fn try_from(torrent: ProwlarrTorrent) -> Result<Self> {
        Ok(Self {
            name: torrent.title,
            torrent: None,
            magnet: torrent.guid,
            seeders: torrent.seeders,
            leechers: torrent.leechers,
            downloads: 0,
            size: torrent.size.to_string(),
            source: Source::Prowlarr,
        })
    }
}
