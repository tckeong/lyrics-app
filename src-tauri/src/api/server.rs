use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::spotify_api::spotify_credentials::{ClientAuth, TokenAuth};
use crate::AppState;

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub auth: Option<TokenAuth>,
    pub access_token: Option<String>,
}

impl Token {
    pub fn new() -> Self {
        Token {
            auth: None,
            access_token: None,
        }
    }

    pub async fn callback(
        state: web::Data<AppState>,
        query: web::Query<CallbackQuery>,
    ) -> impl Responder {
        let code = &query.code;
        let client_auth = ClientAuth::get_credentials(code.to_string()).unwrap();
        let auth = TokenAuth::auth(client_auth).await.unwrap();

        {
            let mut new_token = Token::new();
            new_token.auth = Some(auth.clone());
            new_token.access_token = Some(auth.get_access_token());
            state.token.lock().unwrap().replace(new_token);
        }

        HttpResponse::Ok().body("Authorization successful! Please close this window.")
    }
}
