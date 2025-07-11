pub mod authentication;
pub mod song_details;

use crate::AppState;
use tauri::State;

pub async fn get_token_from_state(state: &State<'_, AppState>) -> Option<String> {
    let mut token = {
        let mut token_guard = state.token.lock().unwrap();
        token_guard.as_mut().cloned()?
    };

    let auth = token.auth.as_mut().unwrap();

    if let Ok(true) = auth.check().await {
        token.access_token = Some(auth.get_access_token());
        state.token.lock().unwrap().replace(token.clone());
    }

    token.access_token.clone()
}
