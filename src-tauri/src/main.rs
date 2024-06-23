// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod controllers;
mod models;

use actix_web::{rt, web, App, HttpServer};
use api::server::Token;
use api::spotify_api::{self};
use controllers::{login, login_test};
use std::{
    env,
    sync::{Arc, Mutex},
};

fn main() {
    std::thread::spawn(move || {
        let token = Arc::new(Mutex::new(Token::new()));

        rt::System::new().block_on(async move {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(token.clone()))
                    .route("/callback", web::get().to(Token::callback))
                    .route("/token", web::post().to(Token::get_token))
            })
            .bind("127.0.0.1:8081")
            .unwrap()
            .run()
            .await
            .unwrap();
        });
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, login_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
