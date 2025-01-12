use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lrc {
    #[serde(rename = "lyric")]
    pub lyrics: String,
}

#[derive(Debug, Deserialize)]
pub struct LyricsResponse {
    #[serde(rename = "pureMusic")]
    pub _pure_music: Option<bool>,
    #[serde(rename = "lrc")]
    pub lrc: Lrc,
}
