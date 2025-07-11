pub mod spotify_credentials;

use crate::models::{CurrentlyPlayingTrack, User};
use reqwest::Client;
use std::error::Error;

pub struct SpotifyApi {
    token: Option<String>,
    client: Client,
}

impl SpotifyApi {
    pub fn new(token: Option<String>) -> Self {
        SpotifyApi {
            token,
            client: Client::new(),
        }
    }

    pub async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        self.token.clone().ok_or_else(|| "No token found!".into())
    }

    pub async fn get_user(&self) -> Result<User, Box<dyn Error>> {
        let token = self.get_token().await?;
        let response = self
            .client
            .get("https://api.spotify.com/v1/me")
            .bearer_auth(token)
            .send()
            .await?
            .json::<User>()
            .await?;

        Ok(response)
    }

    pub async fn get_current_playing(&self) -> Result<CurrentlyPlayingTrack, String> {
        let token = self.get_token().await.map_err(|_| "No token found!")?;
        let response = self
            .client
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .bearer_auth(token)
            .send()
            .await
            .map_err(|_| "No song is currently playing!")?
            .json::<CurrentlyPlayingTrack>()
            .await
            .map_err(|_| "Parsing Error")?;

        Ok(response)
    }

    pub async fn get_current_img(&self) -> Result<String, String> {
        let current_track = self.get_current_playing().await?;

        if let Some(track) = current_track.item {
            Ok(track.album.images[0].url.clone())
        } else {
            Err("No image found!".to_string())
        }
    }

    pub async fn get_title_artist(&self) -> Result<(String, String), String> {
        let current_track = self.get_current_playing().await?;

        if let Some(track) = current_track.item {
            Ok((track.name.clone(), track.artists[0].name.clone()))
        } else {
            Err("No song found!".to_string())
        }
    }

    pub async fn get_current_id(&self) -> Result<String, String> {
        let current_track = self.get_current_playing().await?;

        if let Some(track) = current_track.item {
            Ok(track.id.clone())
        } else {
            Err("No song found!".to_string())
        }
    }

    pub async fn get_play_status(&self) -> Result<bool, String> {
        let current_track = self.get_current_playing().await?;

        Ok(current_track.is_playing)
    }

    pub async fn get_progress_ms(&self) -> Result<u64, String> {
        let current_track = self.get_current_playing().await?;

        if let Some(progress_ms) = current_track.progress_ms {
            Ok(progress_ms)
        } else {
            Err("No progress found!".to_string())
        }
    }

    pub async fn get_duration(&self) -> Result<u64, String> {
        let current_track = self.get_current_playing().await?;

        if let Some(track) = current_track.item {
            Ok(track.duration)
        } else {
            Err("No duration found!".to_string())
        }
    }
}
