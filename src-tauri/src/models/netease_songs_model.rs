use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Ar {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OriginSongSimpleData {
    pub name: String,
    #[serde(rename = "songId")]
    pub id: u32,
    #[serde(rename = "artists")]
    pub artists: Vec<Ar>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Song {
    pub name: String,
    pub id: u32,
    #[serde(rename = "originSongSimpleData")]
    pub origin: Option<OriginSongSimpleData>,
    #[serde(rename = "ar")]
    pub ar: Vec<Ar>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SongsResult {
    #[serde(rename = "songs")]
    pub songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SongsResponse {
    #[serde(rename = "result")]
    pub result: SongsResult,
}