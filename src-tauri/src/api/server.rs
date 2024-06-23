use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

use crate::spotify_api::spotify_credentials::{AuthResponse, ClientAuth};

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    state: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Token {
    access_token: Option<String>,
}

impl Token {
    pub fn new() -> Self {
        Token { access_token: None }
    }

    pub async fn callback(
        token: web::Data<Arc<Mutex<Token>>>,
        query: web::Query<CallbackQuery>,
    ) -> impl Responder {
        let code = &query.code;
        let client_auth = ClientAuth::get_credentials(code.to_string()).unwrap();
        let auth = AuthResponse::auth(client_auth).await.unwrap();

        {
            let mut token = token.lock().unwrap();
            token.access_token = Some(auth.get_access_token());
        }

        HttpResponse::Ok().body("Authorization successful! Please close this window.")
    }

    pub async fn get_token(
        token: web::Data<Arc<Mutex<Token>>>,
        req: web::Json<TokenRequest>,
    ) -> impl Responder {
        if req.state == "122_)%dhA33sdbu@#$" {
            let token = token.lock().unwrap();
            if let Some(token) = token.access_token.clone() {
                let data = json!({
                    "token": token
                });
                HttpResponse::Ok().json(data)
            } else {
                HttpResponse::Unauthorized().body("")
            }
        } else {
            HttpResponse::Unauthorized().body("")
        }
    }
}
