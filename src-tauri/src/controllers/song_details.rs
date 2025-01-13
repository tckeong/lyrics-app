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