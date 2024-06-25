use crate::models::{AuthResponse, RefreshResponse};
use chrono::{DateTime, Utc};
use reqwest::Client;
use std::{env, error::Error};

pub struct ClientAuth {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Debug, Clone)]
pub struct TokenAuth {
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    time_stamp: DateTime<Utc>,
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

impl TokenAuth {
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

        let result = TokenAuth {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            time_stamp: Utc::now(),
        };

        Ok(result)
    }

    pub async fn check(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.time_stamp.timestamp() + 3000 > Utc::now().timestamp() {
            self.access_token = self.refresh().await?;
            self.time_stamp = Utc::now();
        }

        Ok(true)
    }

    pub async fn refresh(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let client_id = self.client_id.as_str();
        let client_secret = self.client_secret.as_str();
        let refresh_token = self.refresh_token.as_str();
        let forms = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
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
            .json::<RefreshResponse>()
            .await?;

        Ok(response.access_token)
    }

    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
}
