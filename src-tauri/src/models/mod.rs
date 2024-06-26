use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    _token_type: String,
    #[serde(rename = "scope")]
    _scope: String,
    #[serde(rename = "expires_in")]
    _expires_in: u64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    _token_type: String,
    #[serde(rename = "scope")]
    _scope: String,
    #[serde(rename = "expires_in")]
    _expires_in: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Followers {
    href: Option<String>,
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    height: Option<u32>,
    pub url: String,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub display_name: Option<String>,
    external_urls: ExternalUrls,
    #[serde(rename = "followers")]
    _followers: Followers,
    href: String,
    id: String,
    #[serde(rename = "images")]
    _images: Vec<Image>,
    #[serde(rename = "type")]
    _user_type: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub token: String,
}

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
    pub external_urls: ExternalUrls,
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
}

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingTrack {
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u64>,
    pub is_playing: bool,
    #[serde(rename = "item")]
    pub item: Option<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Lrc {
    #[serde(rename = "lyric")]
    pub lyrics: String,
}

#[derive(Debug, Deserialize)]
pub struct LyricsResponse {
    #[serde(rename = "pureMusic")]
    pub pure_music: Option<bool>,
    #[serde(rename = "lrc")]
    pub lrc: Lrc,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OriginSongSimpleData {
    pub name: String,
    #[serde(rename = "songId")]
    pub id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ar {
    #[serde(rename = "name")]
    pub name: String,
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SavedLyric {
    pub name: String,
    pub artist: String,
    pub img: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SavedLyrics {
    pub lyrics: Vec<SavedLyric>,
}
