pub mod spotify_credentials;

use crate::models::AccessToken;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct SpotifyApi {
    state: String,
    client: Client,
    url: String,
}

impl SpotifyApi {
    pub fn new() -> Self {
        SpotifyApi {
            state: "122_)%dhA33sdbu@#$".to_string(),
            client: Client::new(),
            url: "http://localhost:8081/token".to_string(),
        }
    }

    pub async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        let body = json!({"state": self.state});
        let response = self
            .client
            .post(self.url.clone())
            .json(&body)
            .send()
            .await?
            .json::<AccessToken>()
            .await?;

        Ok(response.token)
    }
}
