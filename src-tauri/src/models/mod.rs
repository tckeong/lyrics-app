use serde::{Deserialize, Serialize};

pub mod netease_lyrics_model;
pub mod netease_songs_model;
pub mod spotify_credentials_model;
pub mod spotify_track_model;
pub mod spotify_user_model;

pub use netease_lyrics_model::LyricsResponse;
pub use netease_songs_model::SongsResponse;
pub use spotify_credentials_model::{AuthResponse, RefreshResponse};
pub use spotify_track_model::CurrentlyPlayingTrack;
pub use spotify_user_model::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    height: Option<u32>,
    pub url: String,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub token: String,
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
