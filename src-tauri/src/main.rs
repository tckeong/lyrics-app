// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod controllers;
mod models;
mod utils;

use actix_web::{rt, web, App, HttpServer};
use api::server::Token;
use api::spotify_api::{self};
use controllers::{
    auth_check, close_window, get_id, get_image_url, get_lyrics, get_lyrics_list, get_play_status,
    get_time, get_username, login, login_test, lyric_window, original_window, save_lyrics,
};
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
        .invoke_handler(tauri::generate_handler![
            login,
            login_test,
            get_username,
            close_window,
            lyric_window,
            original_window,
            get_image_url,
            get_lyrics,
            get_id,
            get_play_status,
            get_time,
            save_lyrics,
            auth_check,
            get_lyrics_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
