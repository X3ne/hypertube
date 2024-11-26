use super::error::Result;
use apistos::ApiComponent;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub enum Source {
    #[serde(rename = "1337x")]
    _1337x,
    Yts,
    Eztv,
    TorrentGalaxy,
    Torlock,
    PirateBay,
    Nyaasi,
    Rarbg,
    Ettv,
    Zooqle,
    KickAss,
    Bitsearch,
    Glodls,
    MagnetDL,
    LimeTorrent,
    TorrentFunk,
    TorrentProject,
    Prowlarr,
    Unknown,
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        match value {
            value if value.contains("1337x") => Source::_1337x,
            value if value.contains("yts") => Source::Yts,
            value if value.contains("eztv") => Source::Eztv,
            value if value.contains("torrentgalaxy") => Source::TorrentGalaxy,
            value if value.contains("torlock") => Source::Torlock,
            value if value.contains("piratebay") => Source::PirateBay,
            value if value.contains("nyaa") => Source::Nyaasi,
            value if value.contains("rarbg") => Source::Rarbg,
            value if value.contains("ettv") => Source::Ettv,
            value if value.contains("zooqle") => Source::Zooqle,
            value if value.contains("kickass") => Source::KickAss,
            value if value.contains("bitsearch") => Source::Bitsearch,
            value if value.contains("glodls") => Source::Glodls,
            value if value.contains("magnetdl") => Source::MagnetDL,
            value if value.contains("limetorrent") => Source::LimeTorrent,
            value if value.contains("torrentfunk") => Source::TorrentFunk,
            value if value.contains("torrentproject") => Source::TorrentProject,
            value if value.contains("prowlarr") => Source::Prowlarr,
            _ => Source::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct Torrent {
    pub name: String,
    pub torrent: Option<String>,
    pub magnet: String,
    pub seeders: i64,
    pub leechers: i64,
    pub downloads: i64,
    pub size: String,
    pub source: Source,
}

pub(super) fn preprocess_name(name: &str) -> String {
    let name = name.to_lowercase();
    let re = Regex::new(r"[^\w\s]").unwrap();
    let cleaned_name = re.replace_all(&name, "");

    cleaned_name.to_string()
}

#[async_trait::async_trait]
pub trait Indexer {
    async fn search(&self, query: &str) -> Result<Vec<Torrent>>;
}
