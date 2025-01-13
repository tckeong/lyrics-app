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