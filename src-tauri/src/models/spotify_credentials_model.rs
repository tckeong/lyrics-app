use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    _token_type: String,
    #[serde(rename = "scope")]
    _scope: String,
    #[serde(rename = "expires_in")]
    _expires_in: u64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    _token_type: String,
    #[serde(rename = "scope")]
    _scope: String,
    #[serde(rename = "expires_in")]
    _expires_in: u64,
}