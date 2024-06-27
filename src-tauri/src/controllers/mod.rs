use tauri::window;

use crate::{api::LyricsAPI, models::SavedLyric, spotify_api::SpotifyApi, utils::Utils};
use reqwest::Url;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
pub fn login(client_id: &str, client_secret: &str) -> Result<String, String> {
    env::set_var("CLIENT_ID", client_id.to_string());
    env::set_var("CLIENT_SECRET", client_secret.to_string());

    let redirect_uri = "http://localhost:8081/callback";
    let scope = "user-read-private user-read-email user-read-currently-playing";
    let query_params = [
        ("response_type", "code"),
        ("client_id", client_id),
        ("scope", scope),
        ("redirect_uri", redirect_uri),
    ];

    let url = Url::parse_with_params("https://accounts.spotify.com/authorize", &query_params)
        .map_err(|_| "Error parsing URL!")?
        .to_string();

    Utils::new()
        .save_authentication(client_id, client_secret)
        .map_err(|e| e.to_string())?;

    Ok(url)
}

#[tauri::command]
pub async fn auth_check() -> Result<(String, String), String> {
    let (client_id, client_secret) = Utils::new()
        .get_authentication()
        .await
        .map_err(|_| "Auth Check failed!".to_string())?;

    Ok((client_id, client_secret))
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

#[tauri::command]
pub async fn get_username() -> Result<String, String> {
    let test = SpotifyApi::new();
    let user = test.get_user().await.map_err(|_| "No user found!")?;
    let name = {
        match user.display_name {
            None => Err("No display name found!"),
            Some(name) => Ok(name),
        }
    }?;

    Ok(name)
}

#[tauri::command]
pub async fn lyric_window(app: tauri::AppHandle) -> Result<(), String> {
    let _ = tauri::WindowBuilder::new(&app, "label", tauri::WindowUrl::App("/lyric".into()))
        .center()
        .title("spotify-lyrics-app -- Lyrics")
        .resizable(false)
        .transparent(false)
        .inner_size(800.0, 600.0)
        .build()
        .unwrap();

    Ok(())
}

#[tauri::command]
pub fn close_window(window: window::Window) {
    window.close().unwrap();
}

#[tauri::command]
pub async fn original_window(app: tauri::AppHandle) {
    let _ =
        tauri::WindowBuilder::from_config(&app, app.config().tauri.windows.get(0).unwrap().clone())
            .build()
            .unwrap();
}

#[tauri::command]
pub async fn get_lyrics() -> Result<String, String> {
    let (title, artist) = SpotifyApi::new().get_title_artist().await?;

    let lyrics = match Utils::new()
        .check_lyrics(title.clone())
        .await
        .map_err(|_| "No lyrics found!".to_string())
    {
        Ok(lyrics) => lyrics,
        Err(_) => {
            LyricsAPI::new()
                .get_songs(title.clone(), artist.clone())
                .await?
                .get_lyrics()
                .await?
        }
    };

    Ok(lyrics)
}

#[tauri::command]
pub async fn get_image_url() -> Result<String, String> {
    let img_url = SpotifyApi::new().get_current_img().await?;

    Ok(img_url)
}

#[tauri::command]
pub async fn get_id() -> Result<String, String> {
    let id = SpotifyApi::new().get_current_id().await?;

    Ok(id)
}

#[tauri::command]
pub async fn get_play_status() -> Result<bool, String> {
    let status = SpotifyApi::new().get_play_status().await?;

    Ok(status)
}

#[tauri::command]
pub async fn get_time() -> Result<u64, String> {
    let time = SpotifyApi::new().get_progress_ms().await?;

    Ok(time)
}

#[tauri::command]
pub async fn get_duration() -> Result<u64, String> {
    let duration = SpotifyApi::new().get_duration().await?;

    Ok(duration)
}

#[tauri::command]
pub async fn save_lyrics() -> Result<(), String> {
    let (title, artist) = SpotifyApi::new().get_title_artist().await?;

    match Utils::new()
        .check_lyrics(title.clone())
        .await
        .map_err(|_| "No lyrics found!".to_string())
    {
        Ok(_) => return Ok(()),
        Err(_) => {
            let lyrics = LyricsAPI::new()
                .get_songs(title.clone(), artist.clone())
                .await?
                .get_lyrics()
                .await?;
            let img = SpotifyApi::new().get_current_img().await?;
            Utils::new()
                .save_lyric(title, artist, img, lyrics)
                .await
                .map_err(|e| e.to_string())?
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn get_lyrics_list() -> Result<Vec<SavedLyric>, String> {
    let data = Utils::new()
        .get_lyrics_list()
        .await
        .map_err(|e| e.to_string())?;

    Ok(data.lyrics)
}
