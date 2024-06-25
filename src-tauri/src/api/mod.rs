use crate::models::{LyricsResponse, SongsResponse};
use serde_json::Value;

pub mod server;
pub mod spotify_api;

#[derive(Debug)]
pub struct LyricsAPI {
    url: String,
    client: reqwest::Client,
    artist: Option<String>,
    title: Option<String>,
    songs_response: Option<SongsResponse>,
    id: Option<u32>,
}

impl LyricsAPI {
    pub fn new() -> Self {
        Self {
            url: "http://localhost:3000".to_string(),
            client: reqwest::Client::new(),
            artist: None,
            title: None,
            songs_response: None,
            id: None,
        }
    }

    fn get_valid_id(self) -> Result<Self, String> {
        let response = self.songs_response.clone().ok_or("No songs found!")?;
        let artist = self.artist.clone().ok_or("No artist found!")?;

        let result: Vec<(String, u32, String)> = response
            .result
            .songs
            .iter()
            .map(|s| {
                if let Some(origin) = &s.origin {
                    (origin.name.clone(), origin.id, s.ar[0].name.clone())
                } else {
                    (s.name.clone(), s.id, s.ar[0].name.clone())
                }
            })
            .collect();

        let filtered_result: Vec<u32> = result
            .iter()
            .filter(|(_, _, ar)| ar == &artist)
            .map(|(_, id, _)| *id)
            .collect();

        if filtered_result.len() <= 0 {
            if result.len() <= 0 {
                Err("No songs found!".to_string())
            } else {
                Ok(Self {
                    url: self.url,
                    client: self.client,
                    artist: self.artist,
                    title: self.title,
                    songs_response: self.songs_response,
                    id: Some(result[0].1),
                })
            }
        } else {
            Ok(Self {
                url: self.url,
                client: self.client,
                artist: self.artist,
                title: self.title,
                songs_response: self.songs_response,
                id: Some(filtered_result[0]),
            })
        }
    }

    async fn artist_name_mapper(&self, artist: String, title: String) -> Result<String, String> {
        let is_ascii = |s: &str| -> bool { s.bytes().all(|b| b.is_ascii()) };

        if is_ascii(&title) {
            return Ok(artist);
        }

        // special case for mayday
        if artist.to_lowercase().replace(" ", "") == "mayday" {
            return Ok("五月天".to_string());
        }

        let from = "en";
        let to = "zh";
        // google translate api
        let url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            from, to, artist
        );

        let response = reqwest::get(&url)
            .await
            .map_err(|_| "Translate Error!".to_string())?
            .text()
            .await
            .map_err(|_| "Translate Error!")?;

        let translated_text: String = serde_json::from_str::<Value>(&response)
            .map_err(|_| "Translate Error!".to_string())?[0][0][0]
            .as_str()
            .unwrap()
            .to_string();

        Ok(translated_text)
    }

    pub async fn get_songs(self, title: String, artist: String) -> Result<Self, String> {
        let artist = self.artist_name_mapper(artist, title.clone()).await?;

        let response = self
            .client
            .get(&format!(
                "{}/cloudsearch?keywords={} {}",
                self.url, title, artist
            ))
            .send()
            .await
            .map_err(|_| "Please start the lyrics server!")?
            .json::<SongsResponse>()
            .await
            .map_err(|_| "No songs found!")?;

        Ok(Self {
            url: self.url,
            client: self.client,
            artist: Some(artist),
            title: Some(title),
            songs_response: Some(response),
            id: self.id,
        }
        .get_valid_id()?)
    }

    pub async fn get_lyrics(&self) -> Result<String, String> {
        let id = self.id.ok_or("No id found!")?;

        let response = self
            .client
            .get(&format!("{}/lyric/?id={}", self.url, id))
            .send()
            .await
            .map_err(|_| "Please start the lyrics server!")?
            .json::<LyricsResponse>()
            .await
            .map_err(|_| "No lyrics found!")?;

        Ok(response.lrc.lyrics)
    }
}
