use crate::AppState;
use crate::{
    api::lyrics_api::LyricsAPI, models::SavedLyric, spotify_api::SpotifyApi, utils::Utils,
};
use tauri::State;

use crate::controllers::get_token_from_state;

#[tauri::command]
pub async fn get_lyrics(state: State<'_, AppState>) -> Result<String, String> {
    let token = get_token_from_state(&state).await;
    let (title, artist) = SpotifyApi::new(token).get_title_artist().await?;

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
pub async fn get_image_url(state: State<'_, AppState>) -> Result<String, String> {
    let token = get_token_from_state(&state).await;
    let img_url = SpotifyApi::new(token).get_current_img().await?;

    Ok(img_url)
}

#[tauri::command]
pub async fn get_id(state: State<'_, AppState>) -> Result<String, String> {
    let token = get_token_from_state(&state).await;
    let id = SpotifyApi::new(token).get_current_id().await?;

    Ok(id)
}

#[tauri::command]
pub async fn get_play_status(state: State<'_, AppState>) -> Result<bool, String> {
    let token = get_token_from_state(&state).await;
    let status = SpotifyApi::new(token).get_play_status().await?;

    Ok(status)
}

#[tauri::command]
pub async fn get_time(state: State<'_, AppState>) -> Result<u64, String> {
    let token = get_token_from_state(&state).await;
    let time = SpotifyApi::new(token).get_progress_ms().await?;

    Ok(time)
}

#[tauri::command]
pub async fn get_duration(state: State<'_, AppState>) -> Result<u64, String> {
    let token = get_token_from_state(&state).await;
    let duration = SpotifyApi::new(token).get_duration().await?;

    Ok(duration)
}

#[tauri::command]
pub async fn save_lyrics(state: State<'_, AppState>) -> Result<(), String> {
    let token = get_token_from_state(&state).await;
    let (title, artist) = SpotifyApi::new(token.clone()).get_title_artist().await?;

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
            let img = SpotifyApi::new(token).get_current_img().await?;
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
