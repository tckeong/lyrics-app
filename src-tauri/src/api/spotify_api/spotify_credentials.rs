use reqwest::Client;
use serde::Deserialize;
use std::{env, error::Error};

pub struct ClientAuth {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    access_token: String,
    #[serde(rename = "token_type")]
    _token_type: String,
    #[serde(rename = "scope")]
    _scope: String,
    #[serde(rename = "expires_in")]
    _expires_in: u64,
    #[serde(rename = "refresh_token")]
    _refresh_token: String,
}

impl ClientAuth {
    pub fn get_credentials(code: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client_id = env::var("CLIENT_ID").expect("");
        let client_secret = env::var("CLIENT_SECRET").expect("");

        if client_id == "" || client_secret == "" {
            return Err("CLIENT_ID or CLIENT_SECRET not found".into());
        }

        Ok(ClientAuth {
            client_id,
            client_secret,
            code,
        })
    }
}

impl AuthResponse {
    pub async fn auth(client_auth: ClientAuth) -> Result<Self, Box<dyn Error>> {
        let client_id = client_auth.client_id.as_str();
        let client_secret = client_auth.client_secret.as_str();
        let code = client_auth.code.as_str();
        let client = Client::new();
        let callback_uri = "http://localhost:8081/callback";
        let forms = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", callback_uri),
        ];

        let response = client
            .post("https://accounts.spotify.com/api/token")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&forms)
            .basic_auth(client_id, Some(client_secret))
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        Ok(response)
    }

    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
}
