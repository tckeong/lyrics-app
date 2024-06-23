use crate::spotify_api::SpotifyApi;
use reqwest::Client;
use serde_json::json;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
pub fn login(client_id: &str, client_secret: &str) {
    env::set_var("CLIENT_ID", client_id.to_string());
    env::set_var("CLIENT_SECRET", client_secret.to_string());
}

// return true if test pass, return false if no token found!
#[tauri::command]
pub async fn login_test() -> Result<bool, bool> {
    let test = SpotifyApi::new();
    let token = test.get_token().await.map_err(|_| false)?;

    if token == "" {
        Err(false)
    } else {
        Ok(true)
    }
}
