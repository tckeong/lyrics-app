use crate::{api::LyricsAPI, models::SavedLyric, spotify_api::SpotifyApi, utils::Utils};
use reqwest::Url;
use std::env;

pub mod authentication;
pub mod song_details;

#[tauri::command]
pub async fn lyric_window(app: tauri::AppHandle) -> Result<(), String> {
    let _ = tauri::WebviewWindowBuilder::new(&app, "label", tauri::WebviewUrl::App("/lyric".into()))
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
pub fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
pub async fn original_window(app: tauri::AppHandle) {
    let _ =
        tauri::WebviewWindowBuilder::from_config(&app, &app.config().app.windows.get(0).unwrap().clone())
            .unwrap()
            .build()
            .unwrap();
}
