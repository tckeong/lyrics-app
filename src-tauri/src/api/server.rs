use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::spotify_api::spotify_credentials::{ClientCredential, SpotifyToken};
use crate::AppState;

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[derive(Clone, Debug)]
pub struct Server {}

impl Server {
    pub async fn callback(
        state: web::Data<AppState>,
        query: web::Query<CallbackQuery>,
    ) -> impl Responder {
        let code = &query.code;
        let client_auth = ClientCredential::get_credentials(code.to_string()).unwrap();
        let spotify_token = SpotifyToken::auth(client_auth).await.unwrap();

        {
            state.token.lock().unwrap().replace(spotify_token);
        }

        HttpResponse::Ok().body("Authorization successful! Please close this window.")
    }
}
