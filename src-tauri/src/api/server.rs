use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};

use crate::spotify_api::spotify_credentials::{ClientAuth, TokenAuth};

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    state: String,
}

#[derive(Clone)]
pub struct Token {
    auth: Option<TokenAuth>,
    access_token: Option<String>,
}

impl Token {
    pub fn new() -> Self {
        Token {
            auth: None,
            access_token: None,
        }
    }

    pub async fn callback(
        token: web::Data<Arc<Mutex<Token>>>,
        query: web::Query<CallbackQuery>,
    ) -> impl Responder {
        let code = &query.code;
        let client_auth = ClientAuth::get_credentials(code.to_string()).unwrap();
        let auth = TokenAuth::auth(client_auth).await.unwrap();

        {
            let mut token = token.lock().unwrap();
            token.auth = Some(auth.clone());
            token.access_token = Some(auth.get_access_token());
        }

        HttpResponse::Ok().body("Authorization successful! Please close this window.")
    }

    pub async fn get_token(
        token_auth: web::Data<Arc<Mutex<Token>>>,
        req: web::Json<TokenRequest>,
    ) -> impl Responder {
        if req.state == "122_)%dhA33sdbu@#$" {
            let mut token_auth = token_auth.lock().unwrap();

            if let Some(mut auth) = token_auth.auth.clone() {
                if auth.check().await.unwrap() {
                    token_auth.access_token = Some(auth.get_access_token());
                }

                let token = token_auth.access_token.clone().unwrap();
                HttpResponse::Ok().json(json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("")
            }
        } else {
            HttpResponse::Unauthorized().body("")
        }
    }
}
