use super::{ExternalUrls, Image};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    external_urls: ExternalUrls,
    href: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub name: String,
    pub release_date: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "external_urls")]
    pub _external_urls: ExternalUrls,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "artists")]
    pub artists: Vec<Artist>,
    pub album: Album,
    #[serde(rename = "duration_ms")]
    pub duration: u64,
}

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingTrack {
    #[serde(rename = "context")]
    pub _context: Option<Context>,
    #[serde(rename = "timestamp")]
    pub _timestamp: u64,
    pub progress_ms: Option<u64>,
    pub is_playing: bool,
    #[serde(rename = "item")]
    pub item: Option<Track>,
}
