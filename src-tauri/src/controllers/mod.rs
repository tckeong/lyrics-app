pub mod authentication;
pub mod song_details;

use crate::AppState;
use tauri::State;

pub async fn get_token_from_state(state: &State<'_, AppState>) -> Option<String> {
    let mut spotify_token = {
        let mut token_guard = state.token.lock().unwrap();
        token_guard.as_mut().cloned()?
    };

    if let Ok(true) = spotify_token.check().await {
        state.token.lock().unwrap().replace(spotify_token.clone());
    }

    spotify_token.get_access_token()
}
